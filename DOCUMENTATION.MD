# Cargo-Smith Documentation

`cargo-smith` is a Rust crate that provides batteries‑included utilities for building production‑ready Actix Web APIs. It offers standardized responses, global error handling, authentication, JWT middleware, validation, pagination, and a modular feature‑based project structure.
## Table of Contents

1. Installation
2. Project Structure
3. Standardized API Responses
4. Global Error Handling
5. API Interceptor (Logging & Timing)
6. JWT Authentication Middleware
7. Authentication & Role Guards
8. Request Validation
9. Pagination Utilities
10. Feature Modules (Users Example)
11. Putting It All Together

## Installation

`cargo-smith` provides two main components:

1. A CLI tool for generating new Actix Web projects with the recommended structure.
2. A Rust library containing the utilities used within the generated projects.


### 1. Install the CLI Generator
```bash
cargo install cargo-smith
```

Once installed, create a new project by running:

```bash
cargo-smith new
```

You will be guided through an interactive questionnaire to configure your project:

```text
? Project name: my-api
```

After answering the prompts, a new directory with your project name will be created, containing:

- A fully configured Cargo.toml
- The modular folder structure (features/, server/, common/)
- Example users module (if selected)
- Environment variable templates (.env.example)
- All middleware and utilities pre‑wired in server/server.rs

Navigate into the new directory and start the server:

```bash
cd my-api
cargo run
```
### 2 Use as a Library (Optional)

If you are adding `cargo-smith` utilities to an existing Actix Web project, add the following to your `Cargo.toml`:

```toml
[dependencies]
cargo-smith = "0.1.0"
```
You can then import the common components:

```rust
use cargo_smith::{
    ApiInterceptor,
    common::response::ApiResponse,
    common::errors::AppError,
    auth::JwtMiddleware,
    common::pagination::{PaginationQuery, PaginatedResponse},
};
```
>Note: The CLI tool automatically adds this dependency to generated projects. Manual installation is only needed when retrofitting an existing codebase.

## Project Structure

The crate encourages a modular feature‑based layout:

```
src/
├── main.rs                 # Entry point – calls server::run()
├── lib.rs                  # Re‑exports common items
├── server/
│   └── server.rs           # HttpServer setup & middleware configuration
├── features/               # Each feature module (users, products, etc.)
│   ├── mod.rs              # Registers all feature routes
│   └── users/
│       ├── mod.rs          # Exports routes::init
│       ├── controller.rs   # Request handlers
│       ├── model.rs        # Data structures
│       ├── routes.rs       # Route configuration for this feature
│       └── service.rs      # Business logic
├── utils/                  # Ready to use
```
## 1. Standardized API Responses

**Response format:**

```json
{
  "status_code": 200,
  "message": "User retrieved successfully",
  "data": { ... }
}
```

**Usage:**

```rust
use cargo_smith::common::response::ApiResponse;

// 200 OK with data
async fn get_user() -> impl Responder {
    let user = User { id: 1, name: "Alice".into() };
    ApiResponse::ok("User retrieved", user)
}

// 201 Created
async fn create_user(body: web::Json<User>) -> impl Responder {
    let new_user = service::create(body.into_inner()).await;
    ApiResponse::created("User created", new_user)
}

// 404 Not Found (message only)
async fn get_user_by_id(path: web::Path<String>) -> impl Responder {
    match service::get_by_id(&path).await {
        Some(user) => ApiResponse::ok("Found", user),
        None => ApiResponse::not_found(format!("User {} not found", path)),
    }
}

// Custom error status
ApiResponse::err(StatusCode::BAD_REQUEST, "Invalid input")
```

**Available methods:**

| Method               | Description                       |
|----------------------|-----------------------------------|
| ok(msg, data)        | 200 OK with payload               |
| created(msg, data)   | 201 Created with payload          |
| success(msg)         | 200 OK message only               |
| err(status, msg)     | Error response with custom status |
| not_found(msg)       | 404 Not Found                     |
| message(status, msg) | Generic message only (any status) |
## 2. Global Error Handling

The `AppError` enum centralises error logic and automatically converts to HTTP responses.

**Definition:**

```rust
pub enum AppError {
    Unauthorized(&'static str),
    Forbidden(&'static str),
    NotFound(String),
    BadRequest(String),
    Internal(String),
    Database(mongodb::error::Error),
}
```

**Automatic Mapping**

| Variant      | HTTP Status | Example                   |
|--------------|-------------|---------------------------|
| Unauthorized | 401         | Invalid/missing token     |
| Forbidden    | 403         | Insufficient permissions  |
| NotFound     | 404         | Resource does not exist   |
| BadRequest   | 400         | Validation/syntax error   |
| Internal     | 500         | Unexpected server error   |
| Database     | 500         | Database operation failed |

**Usage in handlers (via `?` operator):**

```rust
use cargo_smith::common::errors::AppError;

async fn get_user(id: &str) -> Result<User, AppError> {
    let user = db::find_user(id).await?
        .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;
    Ok(user)
}

// In controller:
async fn handler(path: web::Path<String>) -> Result<impl Responder, AppError> {
    let user = service::get_user(&path).await?;
    Ok(ApiResponse::ok("User found", user))
}
```

Because `AppError` implements `ResponseError`, Actix automatically converts `Result<_, AppError>` into proper HTTP responses with the correct status code and JSON body.
## 3. API Interceptor (Logging & Timing)

The `ApiInterceptor` middleware logs every request and adds an `x-response-time-ms` header.

**Setup:**

```rust
use cargo_smith::ApiInterceptor;

HttpServer::new(|| {
    App::new()
        .wrap(ApiInterceptor)   // Add this globally
        .configure(init_routes)
})
```

**Effect:**

- Console output: [API] GET /users - 42ms
- Response header: x-response-time-ms: 42
## 4. JWT Authentication Middleware

The `JwtMiddleware` validates JWTs on every request and injects the decoded claims into request extensions.

**Setup (in `server.rs`):**

```rust
use cargo_smith::auth::JwtMiddleware;

HttpServer::new(move || {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    App::new()
        .wrap(JwtMiddleware::new(jwt_secret))
        .wrap(ApiInterceptor)
        .configure(init_routes)
})
```

**Token Extraction:**

- First tries Authorization: `Bearer <token>` header.
- Falls back to `?token=<token>` query parameter (useful for WebSockets).

**Invalid Tokens:**
- Returns `401 Unauthorized` with a descriptive message (`"missing token"`, `"token expired"`, etc.).
## 5. Request Validation

The `ValidatedJson<T>` extractor automatically validates incoming JSON bodies using the `validator` crate.

**Define a validated struct:**

```rust
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(range(min = 18, max = 120))]
    pub age: u8,
}
```

**Use in handler:**

```rust
use cargo_smith::common::extractors::ValidatedJson;

async fn create_user(body: ValidatedJson<CreateUserRequest>) -> impl Responder {
    let user_data = body.into_inner();
    // At this point validation has already passed
    let new_user = service::create_user(user_data).await;
    ApiResponse::created("User created", new_user)
}
```

If validation fails, the extractor returns `AppError::BadRequest` with the validation messages.
## 6. Pagination Utilities

The crate provides `PaginationQuery` for parsing pagination parameters from query strings and `PaginatedResponse` for consistent paginated output.

**Query Parameters Supported:**

- page – page number (default 1)
- limit – items per page (default 10, max 100)
- sort_by – field to sort by
- sort_order – asc or desc (default desc)
- q – search term
- Any additional filter fields (e.g., ?status=active)

**Usage in Controller:**

```rust
use cargo_smith::common::pagination::{PaginationQuery, PaginatedResponse};

async fn list_users(query: web::Query<PaginationQuery>) -> impl Responder {
    let (items, total) = service::find_users(&query).await;
    let response = PaginatedResponse::new(items, total, &query);
    ApiResponse::ok("Users retrieved", response)
}
```

**Response Structure:**

```json
{
  "status_code": 200,
  "message": "Users retrieved",
  "data": {
    "items": [...],
    "meta": {
      "total": 100,
      "page": 1,
      "limit": 10,
      "total_pages": 10,
      "has_next": true
    }
  }
}
```

## 7. Authentication & Role Guards

`cargo-smith` includes a flexible compile‑time role system built on top of JWT authentication. It allows you to:
- Define a set of roles as an enum (e.g., Admin, User, Guest).
- Automatically generate lowercase string representations ("admin", "user", "guest") that match serialization.
- Enforce role requirements in handlers using a simple AuthGuard extractor.

### Role Traits

Two core traits define the role system:

```rust
pub trait Role:
    Serialize + for<'de> Deserialize<'de> +
    PartialEq + Clone + Send + Sync + Debug + 'static
{
    /// Returns the string representation of the role (e.g., `"admin"`).
    fn as_str(&self) -> &str;
}

pub trait RequiredRole {
    /// The compile‑time constant string identifying the required role.
    const ROLE: &'static str;
}
```

- `Role` is implemented by enums that represent all possible roles in the system.
- `RequiredRole` is implemented by zero‑sized marker types (like `struct Admin;`) that specify a single required role.

### Default Roles

The crate provides a pre‑defined enum `DefaultRoles`:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display, AsRefStr, EnumString, VariantNames)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum DefaultRoles {
    Admin,
    User,
    Guest,
}
```

Three marker structs implement `RequiredRole` with the appropriate lowercase string constants:

| Marker Struct | ROLE Constant |
|---------------|---------------|
| Admin         | "admin"       |
| User          | "user"        |
| Guest         | "guest"       |

You can use them immediately in your handlers (see below).

### Defining Custom Roles
If you need roles beyond the defaults, use the `define_custom_roles!` macro. It generates an enum, marker structs, and all necessary trait implementations in one go.

**Syntax**

```rust
use cargo_smith::define_custom_roles;

define_custom_roles! {
    MyRoles {
        SuperAdmin => SuperAdminMarker,
        Editor     => EditorMarker,
        Viewer     => ViewerMarker,
    }
}
```

**What this expands to:**

- A public enum `MyRoles` with variants `SuperAdmin`, `Editor`, `Viewer`.
- Marker structs `SuperAdminMarker`, `EditorMarker`, `ViewerMarker`.
- Implementation of `Role` for `MyRoles` (using `strum` for string conversion).
- Implementation of `RequiredRole` for each marker, with ROLE set to the lowercase variant name (e.g., `"superadmin"`).

All string conversions use lowercase to match the `#[serde(rename_all = "lowercase")]` attribute applied to the generated enum.

### The `AuthGuard` Extractor
To protect a route, add `AuthGuard<R>` as an extractor parameter, where R is a type that implements `RequiredRole` (e.g., `Admin`, `User`, or your custom marker).

```rust
use cargo_smith::auth::AuthGuard;
use cargo_smith::common::roles::{Admin, User};

async fn delete_user(
    guard: AuthGuard<Admin>,
    path: web::Path<String>,
) -> impl Responder {
    // `guard.claims` contains the decoded JWT claims (e.g., user ID, email, role)
    let claims = guard.claims;
    // Perform admin-only action...
    ApiResponse::ok("User deleted", ())
}
```

**How it works:**

1. `AuthGuard<R>` internally calls `JwtMiddleware` to extract and validate the JWT.
2. It reads the `role` field from the JWT claims (expected claim key: `"role"`).
3. It compares the claim's role string against `R::ROLE`.
4. If they match, the guard succeeds and you can access `guard.claims`.
5. If the token is missing/invalid → `401 Unauthorized`.
6. If the role does not match → `403 Forbidden`.

### JWT Claims Shape

Your login endpoint must issue JWTs with a `role` claim that matches one of the lowercase role strings. For example:

```json
{
  "sub": "user@example.com",
  "exp": 1712345678,
  "role": "admin",
  "iat": 171128382
}
```

### Combining Multiple Guards
Sometimes you need to allow any of several roles. Use `AuthGuard<AnyOf<(Role1, Role2, ...)>>`:

```rust
use cargo_smith::auth::{AuthGuard, AnyOf};
use cargo_smith::common::roles::{Admin, User};

async fn view_dashboard(
    guard: AuthGuard<AnyOf<(Admin, User)>>,
) -> impl Responder {
    // Both Admin and User roles are permitted
    ApiResponse::ok("Dashboard data", ...)
}
```

### No Role Requirement (Authenticated Only)
If you only need to ensure the user is authenticated (any role), use `AuthGuard<NoRole>`:

```rust
use cargo_smith::auth::NoRole;

async fn profile(guard: AuthGuard<NoRole>) -> impl Responder {
    // Any valid token works
    ApiResponse::ok("Profile", guard.claims)
}
```