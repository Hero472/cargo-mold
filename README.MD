# cargo-smith

A high-performance CLI tool for scaffolding Modular Rust & Actix Web applications.

Inspired by the scalability of NestJS, built with the power of Rust.

## The modular Vision

`cargo-smith` is transitioning to a `Feature-First` architecture. Instead of flat folders for all controllers or models, we organize code by `Domain Features`. This ensures your project stays maintainable as it grows from a microservice to an enterprise system.

## Why cargo-smith?

- Modular by Design: Automatically registers new features in a central registry.

- Zero-Boilerplate: Generate a feature with Controller, Service, Model, and Routes in one command.

- Developer Experience: Automated Actix-Web ServiceConfig injection—no more manual route imports.

- Optimized: Designed for Rust-level performance and clean binary footprints.

## Installation

```bash
cargo install cargo-smith
```

## Usage

### 1. Create a New Project

```bash
# Create a new Actix Web project
cargo-smith new

# See all available commands
cargo-smith --help
```

This generates a project with the `Modular` template, including a `.cargo-smith` configuration file to track your project state.

### 2. Generate a New Feature

```bash
# Adds a 'users' module with full boilerplate
cargo-smith feature users
```

## Generated Project Structure

```
my-project/
├── .cargo-smith           # Project metadata and feature registry
├── src/
│   ├── main.rs            # Entry point
│   ├── lib.rs             # Library root
│   ├── server/            # Server configuration
│   │   ├── server.rs      # Server start
│   ├── utils/             # Shared utilities
│   └── features/          # Domain Logic (The Core)
│       ├── mod.rs         # Central Registry [SMITH-MOD]
│       └── users/         # Generated Feature
│           ├── mod.rs     # Feature Entry & Init
│           ├── controller.rs
│           ├── service.rs
│           ├── model.rs
│           └── routes.rs
```

## The Roadmap

[x] Modular Template: Feature-based organization
[x] Automatic Registration: Smart code injection into `features/mod.rs`
[] CRUD Generation: Instant database integration for features
[] Routing Framework: Advanced macro-based routing for cleaner controllers

## Resource Generation

Create complete CRUD APIs in seconds:

```bash
# Generate a users resource with full CRUD operations
cargo-smith generate resource users
or
cargo-smith g resource users

# This creates:
# - Model (src/models/user.rs)
# - Handlers (src/handlers/user_handlers.rs)  
# - Routes (src/routes/user_routes.rs)
# - Automatic route registration

# Available endpoints:
# GET    /api/users
# POST   /api/users
# GET    /api/users/{id}
# PUT    /api/users/{id}
# DELETE /api/users/{id}
```

## JWT Authentication

Built-in authentication system:

```rust
use cargo_smith::auth::AuthService;
use cargo_smith::jwt::JwtMiddleware;

// Create auth service
let auth_service = AuthService::new("secret".to_string());

// Create JWT middleware
let jwt_middleware = JwtMiddleware::new("secret".to_string());

// Use in Actix Web app
App::new()
    .wrap(jwt_middleware)
    .route("/protected", web::get().to(protected_handler))
```

## What's Included

- **Actix Web 4.4** with Tokio runtime
- **Structured modules** for routes, handlers, and server configuration
- **JWT Authentication** with middleware protection
- **Resource Generation** for rapid CRUD API development
- **Environment-based configuration** with dotenvy
- **JSON response** examples
- **Production-ready project structure**

## Join the Journey

This is just the beginning! We're building a comprehensive backend framework that combines Rust's performance with developer experience excellence. Your feedback and contributions will help shape the future of backend development in Rust.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for what's new in each version.