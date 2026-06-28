// IPC type definitions for Tauri commands and events.
// Keep in sync with Rust structs in src-tauri/src/types.rs (snake_case, no rename).

// ─── Vault ───────────────────────────────────────────────────────────────────

export interface VaultInfo {
  path: string;
  note_count: number;
}

// ─── Notes ───────────────────────────────────────────────────────────────────

export interface NoteSummary {
  id: string;
  path: string;
  title: string;
  updated_at: string;
  tags: string[];
}

export interface NoteRecord {
  id: string;
  path: string;
  title: string;
  content: string;
  frontmatter: Record<string, unknown>;
  created_at: string;
  updated_at: string;
}

// ─── Tags ────────────────────────────────────────────────────────────────────

export interface Tag {
  id: string;
  name: string;
  slug: string;
}

// ─── Graph ───────────────────────────────────────────────────────────────────

export interface GraphNode {
  id: string;
  title: string;
  degree: number;
}

export interface DanglingNode {
  id: string;
  name: string;
}

export interface GraphEdge {
  source: string;
  target: string;
}

export interface GraphData {
  nodes: GraphNode[];
  edges: GraphEdge[];
  dangling_nodes: DanglingNode[];
}

// ─── Backlinks ───────────────────────────────────────────────────────────────

export interface BacklinkEntry {
  source_path: string;
  source_title: string;
  snippet: string;
}

// ─── Search ──────────────────────────────────────────────────────────────────

export interface SearchResult {
  note: NoteSummary;
  snippet: string;
}

// ─── LLM ─────────────────────────────────────────────────────────────────────

export interface ChatMessage {
  role: "user" | "assistant";
  content: string;
}

// ─── Event payloads ──────────────────────────────────────────────────────────

export interface IndexProgressPayload {
  pct: number;
  scanned: number;
  total: number;
}

export interface IndexDonePayload {
  note_count: number;
}

export interface LlmTokenPayload {
  text: string;
}

export interface LlmDonePayload {
  citations: string[];
}

export interface RefactorResult {
  updated_count: number;
}
