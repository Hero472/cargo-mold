use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct {{name_pascal_case}} {
    // Add fields here
}

impl {{name_pascal_case}} {
    pub fn new() -> Self {
        Self {
            // Initialize fields
        }
    }
}