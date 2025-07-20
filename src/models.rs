use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::option::Option;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<Category>,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: usize,
    pub name: String,
}
