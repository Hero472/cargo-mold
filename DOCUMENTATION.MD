# New Features & API Documentation

The latest version of `cargo-smith` introduces a robust set of tools for handling errors, security, standardized responses, and data validation.

---

## 1. Standardized API Responses
All responses now follow a consistent JSON structure. Use the `ApiResponse` struct to wrap your data.

- **Success**: `ApiResponse::ok("Success message", data)`
- **Created**: `ApiResponse::created("Resource created", data)`
- **Error**: `ApiResponse::err(StatusCode::BAD_REQUEST, "Error message")`

```rust
// Example in a handler
async fn get_user() -> impl Responder {
    let user = User { id: 1, name: "Alice".into() };
    ApiResponse::ok("User retrieved", user)
}
```

## 2. Global Error Handling

The `AppError` enum centralizes error logic and automatically maps to HTTP status codes

| Error Variant | HTTP Status | Description |
| ------ | -------- | ------ |
| `Unauthorized` | 401 | Missing or invalid token |
| `Forbidden` | 403 | Insufficient permissions |
| `NotFound` | 404 | Resource does not exist |
| `BadRequest` | 400 | Validation or syntax error |
| `Database` | 500 | Automatic conversion from DB errors |

## 3. Authentication & Role-Based Guards

Secure your routes using `Extractors`. If the condition isn't met, the request fails before reaching your logic.

- `AuthGuard`: Requires any valid JWT.
- `RegisterGuard` / `GuestGuard`: Requires specific "kind" claim in the JWT.
- `RequiredRole<Role, Marker>`: Strict Role-Based Access Control (RBAC)

### Applying Guards to your Controller

to protect your `get_all` handler, you simply add the desired **Guard**

---

### Option 1: Protect for any Authenticated User

If you just want to ensure the requester is logged in with a valid JWT: 

```rust
use cargo_smith::AuthGuard;

pub async fn get_all(_guard: AuthGuard) -> impl Responder {
    let data = service::get_all().await;
    ApiResponse::ok("users retrieved succesfully", data)
}
```

### Option 2: Protect for Admins Only

if "getting all users" is a sensitive action that only an Administration should perform:

```rust

```