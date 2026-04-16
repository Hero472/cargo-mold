pub(crate) mod commands;
pub(crate) mod templates;
pub(crate) mod auth;
pub(crate) mod utils;
pub(crate) mod database;
pub(crate) mod common;
pub(crate) mod config;

// Auth & JWT
pub use auth::auth::AuthService;
pub use auth::claims::Claims;
pub use common::middleware::jwt::JwtMiddleware;
pub use common::middleware::interceptor::ApiInterceptor;

// Database
pub use database::mongo::{Db, MongoCollection};

// Errors & Responses
pub use common::errors::AppError;
pub use common::response::ApiResponse;

// Guards & Roles
pub use common::guards::{AuthGuard, GuardClaims};
pub use common::guards::{RequireAdmin, RequireAnyRole, RequireAuth, RequireGuest, RequireUser, HasRole, AnyOf};
pub use common::roles::{Admin, User, Guest, DefaultRoles};

// Pagination
pub use common::extractors::pagination::{PaginatedResponse, PaginationQuery};

// Config
pub use config::Config;