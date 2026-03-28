use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// --- 1. CORE TYPES (Database Agnostic) ---

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    #[default]
    Desc,
}

impl SortDirection {
    pub fn to_int(&self) -> i32 {
        match self {
            Self::Asc => 1,
            Self::Desc => -1,
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct PaginationQuery {
    pub page: Option<u64>,
    pub limit: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortDirection>,
    /// The search term (e.g., ?q=john)
    pub q: Option<String>,
    /// Captures anything else (e.g., ?status=active)
    #[serde(flatten)]
    pub filters: HashMap<String, String>,
}

impl PaginationQuery {
    pub fn get_page(&self) -> u64 { self.page.unwrap_or(1).max(1) }
    pub fn get_limit(&self) -> i64 { self.limit.unwrap_or(10).clamp(1, 100) }
    pub fn skip(&self) -> u64 { (self.get_page() - 1) * self.get_limit() as u64 }
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub total: u64,
    pub page: u64,
    pub limit: i64,
    pub total_pages: u64,
    pub has_next: bool,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub meta: PaginationMeta,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: u64, query: &PaginationQuery) -> Self {
        let limit = query.get_limit();
        let page = query.get_page();
        let total_pages = (total as f64 / limit as f64).ceil() as u64;

        Self {
            items,
            meta: PaginationMeta {
                total,
                page,
                limit,
                total_pages,
                has_next: page < total_pages,
            },
        }
    }
}