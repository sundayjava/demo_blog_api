use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    20
}

impl PaginationParams {
    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.limit
    }

    pub fn validate(&mut self) {
        if self.page < 1 {
            self.page = 1;
        }
        if self.limit < 1 {
            self.limit = 20;
        }
        if self.limit > 100 {
            self.limit = 100;
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    pub total_pages: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: i64, limit: i64, total: i64) -> Self {
        let total_pages = if total > 0 {
            (total as f64 / limit as f64).ceil() as i64
        } else {
            0
        };

        Self {
            data,
            page,
            limit,
            total,
            total_pages,
        }
    }
}
