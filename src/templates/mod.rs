use core::fmt;
use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

pub mod template_engine;
pub mod types;
pub mod contents;
pub mod modular;

#[async_trait]
pub trait Template {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn generate(&self, project_name: &str) -> Result<()>;
    async fn add_feature(&self, feature_name: &str) -> Result<()>; 
}

#[derive(ValueEnum, Copy ,Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TemplateType {
    #[serde(rename = "modular")]          // Serializes AS "modular"
    #[serde(alias = "Modular (Actix-Web)")] // Can also be read FROM "Modular (Actix-Web)"
    Modular,
}

impl fmt::Display for TemplateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateType::Modular => write!(f, "Modular (Actix-Web)")
        }
    }
}

impl TemplateType {
    pub fn create(&self) -> Box<dyn Template> {
        match self {
            TemplateType::Modular => Box::new(modular::ModularTemplate)
        }
    }
}

impl FromStr for TemplateType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "modular" | "modular (actix-web)" => Ok(TemplateType::Modular),
            _ => anyhow::bail!("Unknown template type: '{}'", s),
        }
    }
}