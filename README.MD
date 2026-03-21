# Actix Web Project Generator

A powerful CLI tool for quickly scaffolding new Actix Web projects with production-ready structure and best practices.

## Our Vision

This tool aims to become the **NestJS of the Rust world** - a comprehensive framework and CLI for building efficient, scalable, and maintainable backend systems with Rust. We're building more than just a project generator; we're creating a complete ecosystem for backend development.

## Why cargo-mold?

### Before cargo-mold:
- Spend hours setting up project structure
- Manually configure routes, handlers, modules
- Research best practices for Actix Web
- Write boilerplate code

### After cargo-mold:
- `cargo-mold new my-app` - done in seconds
- Production-ready structure out of the box
- Follows Rust & Actix best practices
- Focus on your business logic, not setup

## The Roadmap

### Phase 1: Project Scaffolding
- Basic Actix Web project generation
- Standardized project structure
- Essential dependencies and configuration

### Phase 2: Advanced Generators
- **Resource Generation**: CRUD endpoints, models, handlers
- **Database Integration**: Diesel, SQLx setup with migrations
- **Authentication**: JWT, OAuth2 boilerplate
- **API Documentation**: OpenAPI/Swagger integration

### Phase 3: Framework Features
- **Dependency Injection**: Type-safe DI system
- **Modules & Providers**: Organized application architecture
- **Middleware System**: Custom middleware generation
- **Configuration Management**: Environment-based configs

### Phase 4: Enterprise Ready
- **Microservices**: Inter-service communication
- **Testing Suite**: Unit, integration, e2e testing setup
- **Deployment**: Docker, Kubernetes configurations
- **Monitoring**: Logging, metrics, health checks

## Installation

```bash
cargo install cargo-smith
```

## Usage

```bash
# Create a new Actix Web project
cargo-smith new

cargo-smith generate resource users
cargo-smith g resource users

# See all available commands
cargo-smith --help
```

## Features

- **Quick Setup** - Generate complete Actix Web projects in seconds
- **Production Structure** - Organized module layout following best practices
- **Ready to Code** - Pre-configured with common dependencies and routes
- **Extensible** - Easy to customize and extend generated projects
- **JWT Authentication** - Built-in auth system with middleware
- **Resource Generation** - Scaffold complete CRUD APIs

## Generated Project Structure

```
my-project/
├── Cargo.toml
├── .cargo-smith
├── .env.example
└── src/
    ├── main.rs
    ├── lib.rs
    ├── routes/
    │   ├── mod.rs
    │   ├── routes.rs
    │   └── user_routes.rs        # Generated with resource command
    ├── handlers/
    │   ├── mod.rs
    │   ├── handlers.rs
    │   └── user_handlers.rs      # Generated with resource command
    ├── models/
    │   ├── mod.rs
    │   ├── models.rs
    │   └── user.rs               # Generated with resource command
    ├── server/
    │   ├── mod.rs
    │   └── server.rs
    └── utils/
        └── mod.rs
```

## Quick Start

```bash
# Generate a new project
cargo-smith new

# Navigate to your new project
cd my-app

# Run the server
cargo run

# Visit http://127.0.0.1:8080/api/hello to see it working!

```
## Resource Generation

Create complete CRUD APIs in seconds:

```bash
# Generate a users resource with full CRUD operations
cargo mold generate resource users

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