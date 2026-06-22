# GraphNotes — Implementation Checklist

**Version 2.0 | June 2026**
**Stack: Tauri 2.0 · Rust · React 18 / TypeScript · SurrealDB (native)**

Each item is tagged `[BE]` (Rust backend), `[FE]` (React frontend), or `[BOTH]` where coordination is needed.

---

## Phase 0 — Project Scaffold

Goal: a running Tauri 2.0 app with Rust backend and React frontend, no features yet.

- [ x] Install prerequisites: Rust toolchain (`rustup`), Node.js 20+, Tauri CLI (`cargo install tauri-cli@^2`)
- [ x] Scaffold project: `cargo tauri init` with React + TypeScript + Vite template
- [x ] Verify `cargo tauri dev` launches a WebView window with the default React page
- [x ] Configure `Cargo.toml` workspace if splitting backend into sub-crates
- [x ] Add Clippy and rustfmt config (`clippy.toml`, `.rustfmt.toml`)
- [x ] Set up `[profile.dev]` and `[profile.release]` in `Cargo.toml` (e.g., `debug = true`, `opt-level = 0` for dev)
- [x ] Add `tauri-plugin-fs`, `tauri-plugin-dialog`, `tauri-plugin-shell` to `Cargo.toml` and register in `main.rs`
- [x ] Configure `tauri.conf.json`: app name, identifier, window defaults, allowed IPC commands (`allowlist` or capabilities)
- [x ] Set up frontend: install Tailwind CSS, shadcn/ui (or Radix UI), configure Vite
- [x ] Create `src/types/ipc.ts` — empty file, establish the pattern for IPC type definitions
- [x ] Create `src-tauri/src/state.rs` — empty `AppState` struct, register with `.manage()` in Tauri builder
- [x ] Confirm hot-reload works: edit a React component, see it update without Rust recompile

**Phase 0 exit criteria:** `cargo tauri dev` runs, window opens, frontend HMR works.

---

## Phase 1 — Core Vault Engine

Goal: open a vault directory, scan .md files, store notes in SurrealDB, display note list.

### 1.1 SurrealDB Setup `[BE]`
- [ ] Add `surrealdb` crate with `kv-surrealkv` feature to `Cargo.toml`
- [ ] Resolve app data path (use `tauri::Manager::path().app_data_dir()`)
- [ ] Initialize DB connection on app startup: `Surreal::new::<SurrealKv>(path)`, use_ns/use_db
- [ ] Define schema via SurrealQL `DEFINE TABLE` / `DEFINE FIELD` / `DEFINE INDEX` statements for `note`, `tag`, `dangling_node`, `links_to`, `tagged_with`
- [ ] Store DB handle in `AppState`

### 1.2 Vault Open `[BOTH]`
- [ ] `[FE]` "Open Vault" button calls `tauri-plugin-dialog` folder picker → passes path to backend
- [ ] `[BE]` Implement `vault_open` Tauri command: accepts path, stores in `AppState`, triggers indexer
- [ ] `[BE]` Implement recursive `.md` file scanner using `tokio::fs::read_dir`
- [ ] `[BE]` Persist last-opened vault path to `tauri-plugin-store` so it reopens on launch
- [ ] `[BE]` Emit `vault_index_progress` events (percent complete) and `vault_index_done` to frontend
- [ ] `[FE]` Listen for progress events and show a loading indicator

### 1.3 Frontmatter and Basic Parsing `[BE]`
- [ ] Add `pulldown-cmark` to `Cargo.toml`
- [ ] Implement frontmatter extractor: regex or manual parse for `---\n...\n---` block at file start
- [ ] Deserialize frontmatter YAML into `serde_json::Value` using `serde_yaml`
- [ ] Implement basic `NoteRecord` struct: `path`, `title` (from frontmatter or filename), `content`, `frontmatter`, `created_at`, `updated_at`
- [ ] Implement upsert: `CREATE OR UPDATE note:⟨id⟩ CONTENT { ... }`

### 1.4 Note List `[BOTH]`
- [ ] `[BE]` Implement `note_list` command: query all notes from SurrealDB, return `Vec<NoteSummary>`
- [ ] `[FE]` Note list sidebar component: displays title and path, sorted by `updated_at`
- [ ] `[FE]` Clicking a note calls `note_read` command and opens it in the editor stub

### 1.5 Note CRUD `[BOTH]`
- [ ] `[BE]` `note_read`: fetch note content by path; read from disk (source of truth), not DB
- [ ] `[BE]` `note_save`: write content to disk → re-parse and update SurrealDB record
- [ ] `[BE]` `note_create`: create `.md` file, insert SurrealDB record, return new note summary
- [ ] `[BE]` `note_delete`: delete `.md` file, remove SurrealDB record and all outgoing edges
- [ ] `[BE]` `note_rename`: rename `.md` file, update SurrealDB record path/title (cascade in Phase 3)

**Phase 1 exit criteria:** can open a vault, see note list, open and save notes. No wikilinks yet.

---

## Phase 2 — Editor and Rendering

Goal: CodeMirror 6 editor for source editing, rendered Markdown preview with KaTeX.

### 2.1 CodeMirror 6 Integration `[FE]`
- [ ] Install: `@codemirror/view`, `@codemirror/state`, `@codemirror/lang-markdown`, `@codemirror/language`
- [ ] Create `Editor.tsx` component wrapping `EditorView` in a `useEffect` / `useRef`
- [ ] Wire `onChange` via `EditorView.updateListener` → save content to React state
- [ ] Handle programmatic content updates (note switching): destroy and re-create `EditorView`, or use `EditorView.dispatch` with a full document replacement transaction
- [ ] Add `@codemirror/highlight` for syntax highlighting theme (light + dark variants)
- [ ] Add Vim keybindings (optional, via `@replit/codemirror-vim`) — useful for power-user audience

### 2.2 Language Packs `[FE]`
- [ ] Install language packs for common languages: `@codemirror/lang-python`, `@codemirror/lang-javascript`, `@codemirror/lang-rust`, `@codemirror/lang-cpp`, `@codemirror/lang-java`, `@codemirror/lang-sql`
- [ ] Configure Markdown mode to use nested language highlighting for fenced code blocks

### 2.3 Markdown Preview `[FE]`
- [ ] Create `Preview.tsx` rendering Markdown to HTML (use `marked` or `unified`/`remark`/`rehype`)
- [ ] Implement split-pane layout: editor left, preview right (resizable via CSS `grid` or a resize library)
- [ ] Toggle between source-only, preview-only, and split-pane modes

### 2.4 KaTeX Integration `[FE]`
- [ ] Install `katex`
- [ ] In the Markdown render pipeline, intercept `$...$` and `$$...$$` tokens before HTML output
- [ ] Render inline math (`$...$`) with `katex.renderToString(expr, { displayMode: false })`
- [ ] Render display math (`$$...$$`) with `katex.renderToString(expr, { displayMode: true })`
- [ ] Handle KaTeX render errors gracefully (show raw expression with error class)

### 2.5 Theme System `[FE]`
- [ ] Define CSS custom properties for all color tokens (background, surface, border, text-primary, text-secondary, accent)
- [ ] Implement `useTheme` hook: reads `prefers-color-scheme`, stores override in `tauri-plugin-store`
- [ ] Apply theme class to `document.documentElement`; all component styles reference CSS variables

**Phase 2 exit criteria:** CodeMirror editor works, live preview shows rendered Markdown and LaTeX.

---

## Phase 3 — Linking Engine

Goal: wikilinks create graph edges, backlinks panel, dangling links, cascade rename.

### 3.1 Wikilink Parser `[BE]`
- [ ] Implement regex-based wikilink extractor operating on note content strings
- [ ] Patterns to match: `[[Target]]`, `[[Target|Alias]]`, `[[Target#Section]]`, `[[Target#^block-id]]`, `![[Target]]` (transclusion)
- [ ] For each match: resolve target note by title (case-insensitive lookup in SurrealDB)
- [ ] If not found: create or upsert `dangling_node` record
- [ ] Create `links_to` edges: `RELATE note:⟨source⟩ -> links_to -> note:⟨target⟩ CONTENT { alias, section_anchor, block_id, line_number }`
- [ ] Run wikilink extraction during vault indexing (Phase 1.2) and on every note save

### 3.2 Backlinks Panel `[BOTH]`
- [ ] `[BE]` `graph_query_backlinks`: SurrealQL `SELECT <-links_to<-note.* FROM note WHERE path = $path`
- [ ] `[BE]` Include context snippet: extract 1-2 lines surrounding the wikilink in the source note
- [ ] `[FE]` `BacklinksPanel.tsx`: collapsible sidebar panel listing each backlink with title + snippet
- [ ] `[FE]` Clicking a backlink navigates to that note (calls `note_read`)

### 3.3 Unlinked Mentions `[BOTH]`
- [ ] `[BE]` `graph_query_unlinked_mentions`: full-text search for note title in all notes; exclude paths that already have `links_to` edge to the current note
- [ ] `[FE]` Display unlinked mentions as a separate section within the Backlinks panel

### 3.4 Dangling Links `[FE]`
- [ ] Render dangling wikilinks in a distinct style (orange text, dashed underline)
- [ ] On click: offer "Create Note" prompt → calls `note_create` with the dangling title

### 3.5 Cascade Link Refactoring `[BE]`
- [ ] Implement `note_rename` fully: after renaming, query all `links_to` edges pointing to old note
- [ ] Spawn Tokio task: for each source note, read file, regex-replace old wikilink patterns, write back
- [ ] Emit `refactor_progress` events, then `refactor_done` with count of files updated
- [ ] `[FE]` Show toast notification on refactor completion

### 3.6 Transclusion `[FE]`
- [ ] In the preview renderer: detect `![[Target]]` and replace with content of target note (fetched via `note_read`)
- [ ] For section transclusion `![[Target#Section]]`: extract content under the specified heading
- [ ] Re-render transcluded content (including its own Markdown and KaTeX)

**Phase 3 exit criteria:** wikilinks render with hover previews, backlinks panel populated, cascade rename works.

---

## Phase 4 — Graph Visualization

Goal: interactive force-directed knowledge graph.

### 4.1 Graph Data Query `[BE]`
- [ ] `graph_query_full`: return all nodes (`id`, `title`, `degree`) and all edges (`source_id`, `target_id`) — consider pagination or summarization for vaults > 5k notes

### 4.2 Force-Directed Graph `[FE]`
- [ ] Choose visualization library: D3.js (`d3-force`) for vaults up to ~2k nodes; Sigma.js + graphology for larger vaults
- [ ] Create `GraphPanel.tsx`: full-screen or panel-sized canvas/SVG, receives node and edge data
- [ ] Implement force simulation: `forceLink`, `forceManyBody`, `forceCenter`
- [ ] Scale node radius by `degree` (number of inbound + outbound edges)
- [ ] Render dangling nodes with dashed border / distinct fill color
- [ ] Click a node: call `note_read` and open note in editor
- [ ] Implement zoom and pan (D3 zoom behavior or Sigma.js camera)
- [ ] Implement node drag (D3 drag or Sigma.js drag plugin)

### 4.3 Tag Filtering `[BOTH]`
- [ ] `[BE]` `tag_list`: return all tags with note count
- [ ] `[FE]` Tag Explorer panel: list all tags, click to filter graph and note list
- [ ] `[FE]` `graph_query_by_tag`: fetch subgraph for selected tag

**Phase 4 exit criteria:** graph renders vault, click-to-navigate works, tag filter applies.

---

## Phase 5 — Search and Navigation

Goal: command palette (Ctrl+P), quick switcher (Ctrl+O), full-text search.

### 5.1 Command Palette `[FE]`
- [ ] Implement `CommandPalette.tsx`: modal overlay triggered by `Ctrl/Cmd+P`
- [ ] Use `Fuse.js` for fuzzy matching over a combined list: note titles, command names, settings
- [ ] Commands to wire: New Note, Open Note, Open Vault, Rename Note, Insert Template, Open Graph, Toggle Theme, Daily Note
- [ ] Keyboard navigation: arrow keys to move selection, Enter to execute, Escape to close

### 5.2 Quick Switcher `[FE]`
- [ ] Implement lighter variant (`Ctrl/Cmd+O`): searches only note titles, no command entries
- [ ] Shows recent notes at the top before query is entered

### 5.3 Full-Text Search `[BE]`
- [ ] Option A: SurrealDB full-text index — `DEFINE INDEX note_content ON note FIELDS content SEARCH ANALYZER ascii BM25`; query with `MATCH`
- [ ] Option B: `tantivy` crate for more sophisticated full-text search (better tokenization, ranking)
- [ ] `[BE]` `search_full_text`: accepts query string, returns matching `NoteSummary` list with highlight snippets
- [ ] `[FE]` Integrate into command palette as a "search vault" action

### 5.4 Daily Notes `[BOTH]`
- [ ] `[BE]` `daily_note_open`: check for note named `YYYY-MM-DD.md` in vault root (or configured folder); create from daily template if absent
- [ ] `[FE]` Bind to keyboard shortcut or command palette entry "Open Daily Note"

**Phase 5 exit criteria:** Ctrl+P command palette works, full-text search returns results.

---

## Phase 6 — Templates System

Goal: template directory, variable substitution, quick-insert via command palette.

- [ ] `[BE]` Scan vault for a `templates/` directory on vault open; index all `.md` files within it as templates
- [ ] `[BE]` Implement `template_list` command: return list of available templates with names and variable lists
- [ ] `[BE]` Implement variable substitution engine: replace `{{title}}`, `{{date}}`, `{{time}}`, `{{tags}}`, `{{cursor}}` in template content
- [ ] `[BE]` `template_insert`: accepts template name + variable values, returns substituted content
- [ ] `[FE]` Template picker modal (accessible from command palette "Insert Template")
- [ ] `[FE]` For templates with unfilled variables: show a small form before inserting
- [ ] `[FE]` Template gallery: preview pane showing rendered template output before insert

**Phase 6 exit criteria:** templates render and insert correctly with variable substitution.

---

## Phase 7 — LLM Integration

Goal: configurable LLM providers, note-context chat panel, inline completions.

### 7.1 Provider Configuration `[BOTH]`
- [ ] `[FE]` Settings panel: add LLM provider dropdown (OpenAI-compatible, Anthropic, Ollama), base URL, model name
- [ ] `[BE]` Store API keys using OS credential store via `keyring` crate (never write to file)
- [ ] `[BE]` `llm_config_save` / `llm_config_load` commands

### 7.2 Chat Panel `[BOTH]`
- [ ] `[FE]` `ChatPanel.tsx`: sidebar panel with message history and input box
- [ ] `[BE]` `llm_chat_send` command: accepts message + current note content as context
- [ ] `[BE]` Stream response tokens: emit `llm_token` events per token
- [ ] `[FE]` Append tokens to chat UI on event receipt; show streaming cursor
- [ ] `[BE]` `llm_chat_cancel`: abort the in-flight request (use `tokio_util::sync::CancellationToken`)
- [ ] `[FE]` Cancel button visible during streaming

### 7.3 Inline Completions `[BOTH]`
- [ ] `[BE]` `llm_complete_inline`: accepts current paragraph text as prefix, returns completion string
- [ ] `[FE]` Trigger on `Ctrl/Cmd+Space` (or configurable shortcut) in CodeMirror
- [ ] `[FE]` Show completion as ghost text; `Tab` to accept, `Escape` to dismiss

### 7.4 Note Commands `[BOTH]`
- [ ] `[BE]` `llm_summarize`: summarize current note content; return summary string
- [ ] `[BE]` `llm_suggest_tags`: suggest tag names from note content; return `Vec<String>`
- [ ] `[FE]` Wire both commands to command palette entries

**Phase 7 exit criteria:** chat panel works end-to-end with streaming, inline completions insert text.

---

## Phase 8 — GraphRAG

Goal: vector embeddings stored in SurrealDB, RAG-augmented LLM responses with citations.

- [ ] `[BE]` Add `DEFINE INDEX chunk_hnsw ON embedding_chunk FIELDS vector HNSW DIMENSION 1536` to schema
- [ ] `[BE]` Implement note chunker: split note content into paragraphs or fixed-size chunks with overlap
- [ ] `[BE]` `embeddings_generate`: for each note chunk, call embedding API (OpenAI `text-embedding-3-small` or compatible), store in `embedding_chunk` table
- [ ] `[BE]` Run embedding generation as a background Tokio task after initial vault index; update incrementally on note save
- [ ] `[BE]` Implement vector similarity search: `SELECT * FROM embedding_chunk WHERE vector <|10|> $query_vec ORDER BY vector ASC`
- [ ] `[BE]` Implement graph expansion: for top-k chunks, traverse `links_to` edges 1-2 hops to gather additional context
- [ ] `[BE]` Assemble RAG prompt context: rank and deduplicate chunks, inject into LLM system prompt
- [ ] `[BE]` Include source note paths in `llm_done` event payload
- [ ] `[FE]` Render citations in chat UI as clickable links to source notes
- [ ] `[FE]` Settings: toggle GraphRAG on/off, configure chunk size, graph traversal depth

**Phase 8 exit criteria:** LLM responses are grounded in vault content with clickable source citations.

---

## Phase 9 — Android / Mobile

Goal: the same app running on Android via Tauri Mobile.

- [ ] Install Android SDK, NDK, and add Android Rust targets: `rustup target add aarch64-linux-android`
- [ ] Run `cargo tauri android init` and verify project structure
- [ ] Test `cargo tauri android dev` on an Android emulator
- [ ] Audit UI for touch interactions: ensure tap targets are ≥ 44px, swipe gestures work for panel navigation
- [ ] Handle mobile keyboard: ensure CodeMirror works with on-screen keyboard (virtual keyboard may resize the WebView)
- [ ] Test vault selection: Android folder picker via `tauri-plugin-dialog`
- [ ] Validate file watching on Android: `notify` may need polling mode; test for correctness
- [ ] Verify SurrealDB data directory path on Android (`app.app_data_dir()` returns correct path)
- [ ] Test LLM chat panel on small screen: consider bottom-sheet modal layout
- [ ] Run `cargo tauri android build` and install APK on physical device
- [ ] Test cold start time (SurrealDB init + vault index): must be acceptable on mid-range Android hardware

**Phase 9 exit criteria:** APK installs and runs on Android 9+; core note viewing/editing works.

---

## Phase 10 — Polish, Performance, and Hardening

Goal: production-quality experience across all platforms.

### 10.1 Performance
- [ ] Profile vault indexing time for 10k notes; optimize SurrealDB batch upserts (use `INSERT` with array payload)
- [ ] Measure note-open latency; confirm < 100ms for notes under 50KB
- [ ] Profile graph render at 2k and 5k nodes; switch to Sigma.js if D3 drops below 30fps
- [ ] Lazy-load graph data: stream node/edge batches rather than one large JSON payload
- [ ] Optimize Tauri command serialization: avoid cloning large `content` strings unnecessarily

### 10.2 Keyboard Completeness
- [ ] Audit all features for keyboard accessibility
- [ ] Implement global keyboard shortcut registry (visible in command palette as hints)
- [ ] Tab-trap management: ensure focus stays within modals; restore focus on close

### 10.3 Error Handling and Recovery
- [ ] All Tauri commands return `Result<T, String>`; all errors surfaced in frontend as toast notifications
- [ ] Implement `vault_rebuild` command: drop all SurrealDB tables and re-index from raw `.md` files
- [ ] Expose "Rebuild Index" option in settings for manual recovery
- [ ] Validate .md file encoding on read; handle non-UTF-8 files gracefully with error message

### 10.4 Testing
- [ ] Unit tests for parser (frontmatter extractor, wikilink regex, template substitution) — `cargo test`
- [ ] Integration tests for SurrealDB CRUD and graph queries using `surrealdb::engine::local::Mem` (in-memory, no disk I/O needed in tests)
- [ ] Frontend component tests (Vitest + React Testing Library) for Editor, Preview, CommandPalette
- [ ] End-to-end test via `tauri-driver` or Playwright for critical paths (vault open, note save, link resolution)

### 10.5 Distribution
- [ ] Windows: configure NSIS installer with custom icons, desktop shortcut, startup option
- [ ] Linux: AppImage with `.desktop` entry; optionally `.deb` for Debian/Ubuntu
- [ ] Android: configure signing keystore; test on multiple Android versions
- [ ] Auto-update: configure `tauri-plugin-updater` with update endpoint (optional)
