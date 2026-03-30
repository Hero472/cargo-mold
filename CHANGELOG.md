# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-03-30

### Added

- Domain-Driven Reorganization: Moved all template logic from commands to `src/templates/modular/` for better maintainability.

- Feature Command: Replaced generic `resource` with `feature` command to support NestJS-style modularity.

- Smart Injection: Implemented `[SMITH-MOD]` and `[SMITH-INIT]` markers for automated Actix-Web route registration.

- Config-Driven Logic: The CLI now reads `.cargo-smith` to determine the template type and behavior dynamically.

### Changed

- Internal Architecture: Refactored the core to be domain-oriented rather than action-oriented.
- Template Trait: Enhanced Template trait with add_feature capabilities.
- Registry System: Features are now self-contained modules that export an init function for Actix.

### Removed

- Legacy Commands: Deprecated the old resource command in favor of the more robust feature system.

## [0.3.2] - 2026-03-21

### Fixed
- Fixed some template versions and names from `cargo_mold` to `cargo_smith` (Dumb fix) 

## [0.3.1] - 2026-03-21

### Fixed
- Fixed the `README.MD` to match actual development direction

## Changed
- Updated dependencies

## [0.3.0] - 2025-12-02

### Added
- **New template system**: Completely rewrote codebase to support modular, file-based templates
- **Template engine**: New engine handles variable substitution and file generation
- **Future-ready**: Architecture now supports multiple template types
- **NestJS-style project generation**: Added a new feature that lets you generate a project using a NestJS-like modular design

### Changed
- **Code organization**: Separated all template content from Rust code and utils folder is now a file
- **Maintainability**: Templates can now be edited without recompiling the binary

### Fixed
- Fixed command name from `cargo mold` to `cargo-smith` in documentation

### Tweaks
- There is no more emojis in the code :)
- No more cargo-mold, now it's cargo-smith

## [0.2.1] - 2025-09-29

### Fixed
- Generated projects now use correct cargo-mold version in Cargo.toml dependencies

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