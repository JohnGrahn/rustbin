use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct PasteData {
    pub id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}