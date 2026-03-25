[package]
name = "{{name}}"
version = "0.1.0"
edition = "2024"

[dependencies]
cargo-smith = "{{cargo_version}}"
actix-web = "4.13"
tokio = { version = "1.50", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.149"

[lib]
name = "{{name_snake_case}}"
path = "src/lib.rs"
