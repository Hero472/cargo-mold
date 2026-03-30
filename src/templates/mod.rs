use core::fmt;

use anyhow::Result;
use async_trait::async_trait;
use clap::ValueEnum;

pub mod new;
pub mod resource;
pub mod template_engine;
pub mod types;
pub mod contents;

use crate::templates::new::{modular};

#[async_trait]
pub trait Template {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn generate(&self, project_name: &str) -> Result<()>;
}

#[derive(ValueEnum, Clone, Debug)]
pub enum TemplateType {
    Modular
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