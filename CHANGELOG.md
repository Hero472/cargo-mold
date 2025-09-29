# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Improved project structure validation
- Enhanced error messages for better user experience

### Fixed
- Route insertion logic in generated projects
- Module file updates during resource generation

## [0.2.0] - 2025-09-29

### Added
- **Resource generator command** (`cargo mold generate resource <name>`)
- **Automatic route registration**: for generated resources
- **JWT Authentication System**: Complete auth with middleware
- **Resource Generation**: CRUD resource scaffolding
- **Project Validation**: `.cargo-mold` file for project tracking
- **Environment-based Configuration**: JWT secrets from environment

### Changed
- Improved project structure validation

## [0.1.0] - 2025-09-28

### Added
- Initial release
- Project scaffolding with `cargo mold new <name>`
- Basic Actix Web project structure
- Hello check and example routes