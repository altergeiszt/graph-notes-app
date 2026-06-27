use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VaultInfo {
    pub path: String,
    pub note_count: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NoteSummary {
    pub id: String,
    pub path: String,
    pub title: String,
    pub updated_at: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NoteRecord {
    pub id: String,
    pub path: String,
    pub title: String,
    pub content: String,
    pub updated_at: String,
    pub created_at: String,
    pub frontmatter: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BacklinkEntry {
    pub id: String,
    pub path: String,
    pub title: String,
    pub snippet: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProgressPayload {
    pub pct: u8,
    pub scanned: u64,
    pub total: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefactorResult {
    pub updated_count: u32,
}
