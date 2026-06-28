// Structs below that are not yet constructed are intentionally forward-declared
// for upcoming graph, search, and progress-reporting phases.
#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VaultInfo {
    pub path: String,
    pub note_count: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteSummary {
    pub id: String,
    pub path: String,
    pub title: String,
    pub updated_at: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
pub struct Tag {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphNode {
    pub id: String,
    pub title: String,
    pub degree: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DanglingNode {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub dangling_nodes: Vec<DanglingNode>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub note: NoteSummary,
    pub snippet: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BacklinkEntry {
    pub source_path: String,
    pub source_title: String,
    pub snippet: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgressPayload {
    pub pct: u8,
    pub scanned: u64,
    pub total: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefactorResult {
    pub updated_count: u32,
}
