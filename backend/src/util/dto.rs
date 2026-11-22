use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PageQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub order: crate::util::Order,
}

impl PageQuery {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> u64 {
        self.per_page.unwrap_or(10).clamp(1, 100)
    }

    pub fn offset(&self) -> u64 {
        (self.page() - 1) * self.per_page()
    }
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub total_pages: u64,
}

impl<T> Paginated<T> {
    pub fn new(
        data: Vec<T>,
        page: u64,
        per_page: u64,
        total: u64,
    ) -> Self {
        let total_pages = if per_page == 0 {
            0
        } else {
            total.div_ceil(per_page)
        };

        Self {
            data,
            page,
            per_page,
            total,
            total_pages,
        }
    }
}