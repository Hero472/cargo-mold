use crate::features::{{name}}::model::{{name_pascal_case}};

pub async fn get_all() -> Vec<String> {
    // Logic to fetch from database would go here
    vec!["item1".to_string(), "item2".to_string()]
}

pub async fn get_by_id(id: &str) -> Option<String> {
    // Logic to fetch a single item by ID
    if id == "1" {
        Some("item_data".to_string())
    } else {
        None
    }
}

pub async fn create(data: {{name_pascal_case}}) -> {{name_pascal_case}} {
    // Logic to save to database
    data
}