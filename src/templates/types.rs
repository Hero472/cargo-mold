use serde::{Deserialize, Serialize};

// CargoSmith

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoSmith {
    pub project: Project,
    pub generated: Generated,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub smith_version: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Generated {
    pub features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub template: String,
}

// CargoToml

#[derive(Debug, Deserialize)]
pub struct CargoToml {
    pub package: Package,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub version: String,
}