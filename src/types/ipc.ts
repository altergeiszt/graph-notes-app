// IPC type definitions for Tauri commands and events.
// Keep in sync with the Rust structs in src-tauri/src/.

// ─── Vault ───────────────────────────────────────────────────────────────────

export interface VaultInfo {
  path: string;
  noteCount: number;
}

// ─── Notes ───────────────────────────────────────────────────────────────────

export interface NoteSummary {
  id: string;
  path: string;
  title: string;
  createdAt: string; // ISO 8601
  updatedAt: string;
}

export interface NoteRecord extends NoteSummary {
  content: string;
  frontmatter: Record<string, unknown>;
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
  danglingNodes: DanglingNode[];
}

// ─── Backlinks ───────────────────────────────────────────────────────────────

export interface BacklinkEntry {
  id: string;
  path: string;
  title: string;
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
  noteCount: number;
}

export interface LlmTokenPayload {
  text: string;
}

export interface LlmDonePayload {
  citations: string[];
}

export interface RefactorDonePayload {
  updatedCount: number;
}
