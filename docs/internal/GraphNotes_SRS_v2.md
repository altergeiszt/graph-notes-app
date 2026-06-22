**SOFTWARE REQUIREMENTS SPECIFICATION**


**GraphNotes**

*Local-First, Graph-Centric Knowledge Management System*

Built with Rust  ·  Tauri 2.0  ·  React/TypeScript  ·  SurrealDB


| **Document Version** | 2.0 |
| - | - |
| **Status** | Draft |
| **Date** | June 2026 |
| **Classification** | Confidential |
| **Technology Stack** | Rust, Tauri 2.0, React/TypeScript, SurrealDB (Native Embedded) |
| **Deployment Target** | Windows 10/11, Linux, Android (Tauri Mobile) |



# **1. Introduction**

## **1.1 Purpose**

This Software Requirements Specification (SRS) describes the complete functional and non-functional requirements for GraphNotes — a local-first, graph-centric knowledge management application. It is intended for use by architects, engineers, and stakeholders involved in the design, development, and validation of the system.


## **1.2 Scope**

GraphNotes is a next-generation note-taking and knowledge management system that combines the best conventions of tools like Obsidian and Foam with:

- A high-performance Rust backend engine for parsing, indexing, and data management

- SurrealDB as a natively embedded graph database for relationship modelling, querying, and GraphRAG

- Integrated Large Language Model (LLM) capabilities for intelligent note interaction

- Graph Retrieval-Augmented Generation (GraphRAG) for contextually-aware AI responses

- A rich template system and structured workflow tooling

- Full offline operation delivered as a native hybrid desktop and mobile application via Tauri 2.0


## **1.3 Definitions, Acronyms, and Abbreviations**

| **Term** | **Definition** |
| - | - |
| **Tauri 2.0** | **A framework for building native desktop and mobile apps using a Rust backend and web frontend** |
| **IPC** | **Inter-Process Communication — the typed command/event channel between the React frontend and Rust backend in Tauri** |
| **SurrealDB** | **A multi-model database supporting graph, relational, and document data models** |
| **GraphRAG** | **Graph Retrieval-Augmented Generation — using a knowledge graph to ground LLM queries** |
| **LLM** | **Large Language Model — AI model for natural language understanding and generation** |
| **Wikilink** | **Double-bracket \[\[Note Name\]\] hyperlink syntax for internal document references** |
| **Transclusion** | **Embedding live content from one document into another via !\[\[Note Name\]\] syntax** |
| **Frontmatter** | **YAML metadata block at the top of a Markdown file delimited by triple dashes (---)** |
| **Backlink** | **An automatically computed reverse reference from a target note back to its referrers** |
| **Dangling Link** | **A wikilink pointing to a note that does not yet exist (a virtual/placeholder node)** |
| **Block Reference** | **A link targeting a specific paragraph or block via a unique ^block-id identifier** |
| **Force-directed Graph** | **A physics-simulation graph layout where nodes repel and edges attract** |
| **RAG** | **Retrieval-Augmented Generation — augmenting LLM responses with retrieved context** |
| **SurrealKV** | **SurrealDB's embedded key-value storage engine for native (non-WASM) deployments** |
| **Tokio** | **Rust's asynchronous runtime used for non-blocking I/O and background task management** |


## **1.4 References**

- SurrealDB Documentation: https://surrealdb.com/docs

- Tauri 2.0 Documentation: https://v2.tauri.app

- React Documentation: https://react.dev

- Rust Programming Language Reference: https://doc.rust-lang.org

- CommonMark Markdown Specification: https://spec.commonmark.org

- LaTeX Reference: https://www.latex-project.org/help/documentation/



# **2. Overall Description**

## **2.1 Product Perspective**

GraphNotes is a standalone, self-contained application delivered as a native hybrid application using Tauri 2.0. Unlike cloud-dependent tools, it operates entirely on the user's local file system, treating a directory of Markdown files as the single source of truth.


The application packages a React/TypeScript frontend (rendered in a system WebView) with a Rust backend engine connected via Tauri's typed IPC command layer. SurrealDB runs as a natively embedded database using the SurrealKV storage engine, with data persisted to the local application data directory. No embedded web server or network connection is required for core operation.


The application ships as a native installer for Windows and Linux desktop, and as an Android application via Tauri 2.0 Mobile. Optional LLM and sync integrations connect to external services only when explicitly configured by the user.


## **2.2 Product Functions (Summary)**

| **Category** | **Core Functions** |
| - | - |
| **Document Engine** | **Markdown editing, YAML frontmatter, syntax highlighting, LaTeX rendering** |
| **Linking Engine** | **Wikilinks, aliases, backlinks, dangling links, cascade refactoring** |
| **Deep Linking** | **Section targeting, block references, transclusion/embeds** |
| **Graph Database** | **SurrealDB node/edge model, relationship queries, schema evolution** |
| **Knowledge Graph** | **Force-directed graph UI, tag taxonomy, centrality-based node sizing** |
| **LLM Integration** | **Configurable provider (local/cloud), note-aware chat, inline completions** |
| **GraphRAG** | **Semantic vector search over SurrealDB, graph-traversal context retrieval** |
| **Templates** | **Template library, variable substitution, daily note templates, quick-insert** |
| **Workflow Tools** | **Command palette, daily notes engine, fuzzy finder, cascade link refactoring** |
| **Offline / Native** | **Full offline operation, native OS file access, installable app** |


## **2.3 User Classes and Characteristics**

- Knowledge Workers: Users managing large note collections, research, and connected ideas

- Software Developers: Users who value code blocks, technical Markdown, and keyboard-first UX

- Researchers and Academics: Users requiring LaTeX math, bibliography, and citation workflows

- Writers and Creatives: Users leveraging templates, daily journaling, and narrative structures

- Power Users: Users who want LLM assistance and graph-aware AI directly in their notes


## **2.4 Operating Environment**

| **Platform** | **Details** |
| - | - |
| **Windows** | **Windows 10/11 (x64) — primary desktop target; distributed as NSIS or MSI installer** |
| **Linux** | **Ubuntu 20.04+ and derivatives (x64) — distributed as AppImage or .deb package** |
| **Android** | **Android 9+ (API level 28+) via Tauri 2.0 Mobile — distributed as APK/AAB** |
| **macOS** | **macOS 12+ (planned future target; not in initial release scope)** |
| **Local LLM** | **Ollama or compatible local inference server (optional, user-configured)** |
| **Cloud LLM** | **OpenAI, Anthropic, or compatible REST API (optional, user-configured)** |


## **2.5 Design and Implementation Constraints**

- The application MUST be implemented using Tauri 2.0 with a Rust backend and React/TypeScript frontend

- All note data MUST be stored as standard .md files on the user's local file system

- SurrealDB MUST be used as the embedded graph database with the SurrealKV native storage engine

- The application MUST function fully offline without any required network connection

- No user data may be transmitted to external services without explicit user configuration and consent

- The application MUST be packaged as a native installer for Windows and Linux, and an APK/AAB for Android

- All Tauri IPC commands MUST be typed — Rust structs serialized via serde, TypeScript types generated or hand-maintained in sync



# **3. System Features and Requirements**

Each feature is specified with a unique identifier (FR-XXX for functional, NFR-XXX for non-functional), priority (P1=Critical, P2=High, P3=Medium, P4=Low), and description.


## **3.1 Core Architecture**

### **3.1.1 Local File System Vault**

**FR-001 \[P1\]: **The system MUST allow the user to open any directory as a Vault via a native OS directory picker dialog. All .md files within the directory tree become the note corpus.

**FR-002 \[P1\]: **Notes MUST be stored exclusively as UTF-8 encoded .md (Markdown) files on disk.

**FR-003 \[P1\]: **The system MUST watch the vault directory for external file changes using native OS file event notifications and synchronize the in-memory index and SurrealDB graph accordingly.

**FR-004 \[P2\]: **The system MUST support nested subdirectories within a vault without depth limitations.


### **3.1.2 YAML Frontmatter**

**FR-005 \[P1\]: **The parser MUST extract YAML frontmatter blocks (delimited by ---) from the top of each .md file and index the key-value pairs in SurrealDB.

**FR-006 \[P2\]: **Frontmatter fields MUST be exposed as filterable and sortable metadata in all list, search, and query views.

**FR-007 \[P2\]: **Template variables MUST support frontmatter injection (e.g., title, date, tags, aliases).


## **3.2 Linking and Edge Engine**

### **3.2.1 Wikilinks**

**FR-010 \[P1\]: **The parser MUST recognize \[\[Note Name\]\] syntax and create directed edge records in SurrealDB between the source and target note nodes.

**FR-011 \[P1\]: **Wikilink resolution MUST be case-insensitive and support fuzzy matching as a fallback.

**FR-012 \[P1\]: **Link aliases \[\[Target Note|Display Text\]\] MUST render the display text while maintaining the edge pointer to the target node.

**FR-013 \[P1\]: **Dangling links (references to non-existent notes) MUST be tracked as virtual graph nodes in SurrealDB and highlighted distinctly in the UI.

**FR-014 \[P1\]: **Clicking a dangling link MUST offer the user the option to instantiate the note.


### **3.2.2 Backlinks**

**FR-015 \[P1\]: **The system MUST dynamically compute and display a Backlinks panel for the active note, listing all notes that contain a wikilink pointing to it.

**FR-016 \[P2\]: **The Backlinks panel MUST also surface unlinked mentions — plain-text occurrences of the note name that are not yet wikilinks.

**FR-017 \[P2\]: **Each backlink entry MUST display its surrounding context snippet.


### **3.2.3 Cascade Link Refactoring**

**FR-018 \[P1\]: **When a note is renamed or a heading is modified, the system MUST automatically update all wikilinks throughout the vault that reference the old name/heading.

**FR-019 \[P2\]: **Cascade refactoring MUST run as a background Tokio task and report a summary of affected files to the user.


## **3.3 Deep Linking and Transclusion**

**FR-020 \[P1\]: **Section links \[\[Note Name\#Heading\]\] MUST resolve to and render the specific Markdown heading anchor within the target note.

**FR-021 \[P2\]: **Block references MUST be supported via a unique identifier syntax (^block-id) allowing \[\[Note\#^block-id\]\] deep links to specific paragraphs.

**FR-022 \[P2\]: **Transclusion via !\[\[Note Name\]\] MUST render the live textual content of the target note (or section/block) inline within the source note.

**FR-023 \[P2\]: **Transcluded content MUST reflect real-time updates from the source file.


## **3.4 Markdown and LaTeX Rendering**

**FR-030 \[P1\]: **The renderer MUST support CommonMark Markdown specification in full, including headings, emphasis, lists, tables, fenced code blocks, and blockquotes.

**FR-031 \[P1\]: **Fenced code blocks MUST support syntax highlighting for all major programming languages via CodeMirror 6 language packs.

**FR-032 \[P1\]: **Inline LaTeX ($expression$) MUST be parsed and rendered as styled mathematical notation via KaTeX.

**FR-033 \[P1\]: **Display LaTeX ($$expression$$) MUST render as center-aligned block equations via KaTeX.

**FR-034 \[P2\]: **The editor MUST support a dual-pane or live-preview editing mode where Markdown source and its rendered output are visible simultaneously.


## **3.5 LLM Integration**

**FR-040 \[P1\]: **The system MUST provide a configurable LLM provider interface supporting at minimum: OpenAI-compatible REST APIs and local Ollama instances.

**FR-041 \[P1\]: **A sidebar or floating chat panel MUST allow the user to converse with the LLM with the current note injected as context.

**FR-042 \[P2\]: **The system MUST support inline completions triggered by a configurable keyboard shortcut, completing the current sentence or paragraph using LLM inference.

**FR-043 \[P2\]: **LLM-generated content MUST be clearly marked with a distinct visual indicator and kept separate from user-authored content until explicitly accepted.

**FR-044 \[P2\]: **The LLM integration MUST support note summarization, tagging suggestions, and entity extraction commands.

**FR-045 \[P3\]: **LLM prompt templates MUST be editable by the user and stored in the vault as .md files.


## **3.6 GraphRAG**

**FR-050 \[P1\]: **The system MUST generate and store vector embeddings for each note chunk in SurrealDB using a configurable embedding model.

**FR-051 \[P1\]: **When a user asks an LLM question, the GraphRAG engine MUST: (1) perform vector similarity search to retrieve semantically relevant note chunks, (2) expand context by traversing wikilink edges in SurrealDB, and (3) assemble a grounded context for the LLM prompt.

**FR-052 \[P2\]: **GraphRAG responses MUST cite the source notes used as context, with clickable links to those notes.

**FR-053 \[P2\]: **The embedding index MUST be updated incrementally when notes are created, modified, or deleted.

**FR-054 \[P3\]: **The user MUST be able to configure the RAG context window size (number of chunks, graph traversal depth).


## **3.7 Knowledge Graph Visualization**

**FR-060 \[P1\]: **The system MUST provide an interactive 2D force-directed graph visualization panel mapping notes as nodes and wikilinks as directed edges, implemented via D3.js or Sigma.js in the React frontend.

**FR-061 \[P2\]: **Node size MUST scale dynamically based on link density (number of inbound and outbound edges — centrality).

**FR-062 \[P2\]: **The user MUST be able to click any node in the graph to navigate directly to that note.

**FR-063 \[P2\]: **The graph MUST support filtering by tag, date, or frontmatter property.

**FR-064 \[P3\]: **Dangling link nodes MUST be visually distinct (e.g., dashed border, different color).

**FR-065 \[P3\]: **The graph MUST support zoom, pan, and node drag interactions.


## **3.8 Tag System**

**FR-070 \[P1\]: **Inline hashtag syntax (\#tag and \#nested/tag) MUST be parsed and indexed in SurrealDB as taxonomy nodes.

**FR-071 \[P2\]: **A Tag Explorer panel MUST list all tags in the vault with note counts and allow one-click filtering.

**FR-072 \[P2\]: **Tags defined in YAML frontmatter (tags: \[\]) MUST be treated identically to inline \#tags.


## **3.9 Templates System**

**FR-080 \[P1\]: **The system MUST support a dedicated templates directory within the vault. Any .md file in this directory is available as a template.

**FR-081 \[P1\]: **Templates MUST support variable substitution syntax (e.g., \{\{title\}\}, \{\{date\}\}, \{\{time\}\}, \{\{tags\}\}).

**FR-082 \[P2\]: **The system MUST provide a Quick Insert Template command accessible from the command palette.

**FR-083 \[P2\]: **The Daily Notes engine MUST use a configurable daily note template.

**FR-084 \[P2\]: **Templates MUST support conditional blocks and loop constructs for list-based scaffolding.

**FR-085 \[P3\]: **A template gallery view MUST provide previews of available templates before insertion.


## **3.10 Workflow and Ergonomics**

**FR-090 \[P1\]: **A fuzzy command palette MUST be accessible via Ctrl/Cmd+P, providing instant access to file navigation, system commands, and settings.

**FR-091 \[P1\]: **The Daily Notes engine MUST automatically create or open a note named with the current date (YYYY-MM-DD format).

**FR-092 \[P2\]: **The system MUST support a Quick Switcher (Ctrl/Cmd+O) for fuzzy-searching note titles for instant navigation.

**FR-093 \[P2\]: **All primary application actions MUST be accessible via keyboard shortcuts without requiring mouse interaction.

**FR-094 \[P2\]: **The system MUST support split pane editing, allowing two notes to be viewed and edited simultaneously.

**FR-095 \[P3\]: **The system MUST maintain a recently-visited notes history accessible from the command palette.



# **4. External Interface Requirements**

## **4.1 File System Interface**

File system access is handled natively through the Tauri plugin ecosystem:

- tauri-plugin-fs provides cross-platform native file read/write operations

- tauri-plugin-dialog provides native OS directory and file picker dialogs

- File watching uses the notify Rust crate for cross-platform OS-level file event notifications on Windows (ReadDirectoryChangesW), Linux (inotify), and Android (equivalent APIs via Tauri Mobile)

- No browser File System Access API or Origin Private File System (OPFS) is used


## **4.2 LLM Provider Interfaces**

| **Provider** | **Protocol** | **Authentication** | **Notes** |
| - | - | - | - |
| **OpenAI / Compatible** | **HTTPS REST** | **API Key (user-supplied)** | **Chat completions + embeddings via async-openai crate** |
| **Anthropic Claude** | **HTTPS REST** | **API Key (user-supplied)** | **Messages API** |
| **Ollama (local)** | **HTTP REST localhost** | **None required** | **Local inference, no data egress; via ollama-rs crate** |
| **Custom Endpoint** | **HTTPS REST** | **User-configured** | **OpenAI-compatible schema** |


## **4.3 SurrealDB Interface**

- Native embedded SurrealDB using the surrealdb Rust crate with the SurrealKV storage engine

- Data persisted to the platform-specific application data directory (e.g., %APPDATA% on Windows, ~/.local/share on Linux, app data on Android)

- SurrealQL used for all graph queries, vector similarity searches (HNSW index), and schema operations

- No external SurrealDB server process; the database runs in-process within the Rust backend


## **4.4 Application Distribution**

| **Platform** | **Distribution Format** | **Tauri Plugin / Tool** |
| - | - | - |
| **Windows 10/11** | **NSIS installer (.exe) or MSI package** | **cargo tauri build --target x86\_64-pc-windows-msvc** |
| **Linux** | **AppImage or .deb package** | **cargo tauri build --target x86\_64-unknown-linux-gnu** |
| **Android** | **APK / Android App Bundle (AAB)** | **cargo tauri android build** |



# **5. Non-Functional Requirements**

| **ID** | **Category** | **Requirement** |
| - | - | - |
| **NFR-001** | **Performance** | **Vault indexing of 10,000 notes MUST complete within 30 seconds on initial load** |
| **NFR-002** | **Performance** | **Note open and render latency MUST be under 100ms for notes under 50KB** |
| **NFR-003** | **Performance** | **Graph visualization MUST render up to 5,000 nodes at 30fps minimum** |
| **NFR-004** | **Performance** | **Wikilink resolution MUST complete in under 50ms for any vault size** |
| **NFR-005** | **Reliability** | **All user data MUST remain exclusively on local storage; no silent network calls** |
| **NFR-006** | **Reliability** | **A vault corruption recovery mechanism MUST rebuild SurrealDB from raw .md files** |
| **NFR-007** | **Security** | **LLM API keys MUST be stored using the OS credential store (Windows Credential Manager, Linux Secret Service); never in plaintext files** |
| **NFR-008** | **Security** | **No telemetry, analytics, or usage data MUST be collected without explicit user consent** |
| **NFR-009** | **Usability** | **All core operations MUST be completable without a mouse (keyboard-first)** |
| **NFR-010** | **Usability** | **UI MUST support light and dark themes with system preference detection** |
| **NFR-011** | **Compatibility** | **The application MUST run on Windows 10+, Ubuntu 20.04+, and Android 9+ (API 28+)** |
| **NFR-012** | **Maintainability** | **Rust backend and React/TypeScript frontend MUST be architecturally decoupled via Tauri's typed IPC command layer; changes to one layer must not require changes to the other** |
| **NFR-013** | **Scalability** | **The graph data model MUST support vaults of at least 100,000 notes without schema changes** |
| **NFR-014** | **Portability** | **The vault directory MUST be fully portable — moving it to another machine must require zero reconfiguration** |



# **6. Data Model Overview (SurrealDB Schema)**

## **6.1 Node Types (Tables)**

| **Table** | **Type** | **Key Fields** | **Purpose** |
| - | - | - | - |
| **note** | **document + node** | **id, path, title, content, frontmatter, created\_at, updated\_at** | **Core note record** |
| **tag** | **node** | **id, name, slug** | **Tag taxonomy node** |
| **template** | **document** | **id, path, name, variables\[\]** | **Template definition** |
| **embedding\_chunk** | **document** | **id, note\_id, chunk\_index, text, vector** | **Vector chunk for RAG** |
| **daily\_note** | **document** | **id, note\_id, date** | **Daily note registry** |
| **dangling\_node** | **node** | **id, name, referenced\_by\[\]** | **Placeholder for unresolved links** |


## **6.2 Edge Types (Relations)**

| **Relation** | **In → Out** | **Fields** | **Purpose** |
| - | - | - | - |
| **links\_to** | **note → note** | **alias, section\_anchor, block\_id, line\_number** | **Explicit wikilink edge** |
| **tagged\_with** | **note → tag** | **source (inline | frontmatter)** | **Note-to-tag association** |
| **transcluded\_by** | **note → note** | **section\_anchor, block\_id** | **Transclusion relationship** |
| **similar\_to** | **embedding\_chunk → embedding\_chunk** | **score (float)** | **Semantic similarity edge (RAG)** |



# **7. Architecture Overview**

## **7.1 Layered Architecture**

The system is structured into four principal layers. The Tauri 2.0 framework manages the boundary between the Presentation Layer and the Core Engine Layer via a typed IPC channel, replacing any WASM bridge or JS interop complexity.


| **Layer** | **Technology** | **Responsibilities** |
| - | - | - |
| **Presentation Layer** | **React 18 + TypeScript + Vite** | **UI components, editor surface (CodeMirror 6), graph canvas (D3.js / Sigma.js), panels, theme system; hot-reloaded in development via Vite** |
| **IPC Bridge Layer** | **Tauri Commands + Events** | **Type-safe async Rust↔JS message passing; Tauri commands for request/response, Tauri events for streaming (e.g., LLM token streaming, file change notifications)** |
| **Core Engine Layer** | **Rust (native, Tokio async)** | **Markdown parsing, wikilink resolution, vault indexing, file watching (notify), template engine, LLM API clients, GraphRAG pipeline** |
| **Persistence Layer** | **SurrealDB (native, SurrealKV)** | **Graph storage, HNSW vector index, frontmatter index, relation queries; all accessed from the Core Engine Layer only** |


## **7.2 Key Dependencies**

### **7.2.1 Rust Backend Crates**

| **Crate** | **Purpose** |
| - | - |
| **tauri (v2)** | **Application framework; IPC command routing, native window management, plugin system** |
| **surrealdb** | **Embedded graph database with SurrealKV native storage engine** |
| **tokio** | **Async runtime for non-blocking I/O and Tokio-spawned background tasks** |
| **notify** | **Cross-platform file system event watching (inotify / ReadDirectoryChangesW)** |
| **pulldown-cmark** | **CommonMark Markdown parser for Rust** |
| **serde / serde\_json** | **Serialization for all Tauri IPC command arguments and return values** |
| **regex** | **Wikilink and frontmatter pattern matching** |
| **async-openai** | **OpenAI-compatible REST API client (also works with Ollama, Groq, etc.)** |
| **ollama-rs** | **Dedicated Ollama local inference client** |
| **rig + rig-surrealdb** | **LLM orchestration and GraphRAG pipeline with first-party SurrealDB retrieval** |
| **tantivy (optional)** | **Full-text search index for vault-wide text search** |


### **7.2.2 Frontend Dependencies**

| **Package** | **Purpose** |
| - | - |
| **React 18 + TypeScript** | **Component framework and type-safe UI layer** |
| **Vite** | **Development server with HMR and production bundler** |
| **@tauri-apps/api** | **TypeScript bindings for Tauri IPC commands and events** |
| **CodeMirror 6** | **Extensible code/text editor with Markdown and syntax highlighting language packs** |
| **KaTeX** | **LaTeX math expression rendering (inline and display)** |
| **D3.js or Sigma.js** | **Force-directed graph visualization for the knowledge graph panel** |
| **Tailwind CSS** | **Utility-first CSS framework for consistent theming** |
| **Radix UI / shadcn/ui** | **Accessible, unstyled UI primitives for command palette, panels, and dialogs** |
| **Fuse.js** | **Client-side fuzzy search for Quick Switcher and command palette** |


## **7.3 Concurrency Model**

The Tauri Rust backend runs on the Tokio async runtime. All blocking or compute-intensive operations are dispatched as Tokio-spawned background tasks, ensuring the IPC handler thread remains responsive. Background tasks communicate results back to the frontend via Tauri events.


Key background tasks:

- Vault scanning and initial indexing (Tokio task, progress events streamed to UI)

- Incremental file change processing (notify watcher events routed to a Tokio task)

- Cascade link refactoring (spawned Tokio task with completion event)

- LLM streaming response forwarding (Tauri event stream per token)

- Embedding generation and vector indexing (background Tokio task)


## **7.4 IPC Command Pattern**

All frontend-to-backend calls follow the Tauri typed command pattern. Rust handler functions are annotated with \#\[tauri::command\] and registered in the Tauri builder. The TypeScript frontend calls them via invoke() from @tauri-apps/api/core.


Command naming convention: domain\_action (e.g., vault\_open, note\_save, graph\_query\_backlinks, llm\_chat\_send). Return types use Result\<T, String\> on the Rust side, surfaced as Promise\<T\> on the TypeScript side.



# **8. Appendix**

## **8.1 Requirement Priority Summary**

| **Priority** | **Label** | **Count** | **Description** |
| - | - | - | - |
| **P1** | **Critical** | **~25** | **Must be present in MVP; system is non-functional without these** |
| **P2** | **High** | **~30** | **Core differentiating features; should ship in v1.0** |
| **P3** | **Medium** | **~12** | **Quality-of-life improvements; target v1.1+** |
| **P4** | **Low** | **TBD** | **Nice-to-have; future roadmap** |


## **8.2 Feature Traceability Matrix**

| **Feature Area** | **SRS Requirements** |
| - | - |
| **Core Architecture** | **FR-001 to FR-007, NFR-005, NFR-006, NFR-014** |
| **Linking and Edge Engine** | **FR-010 to FR-019** |
| **Deep Linking and Transclusion** | **FR-020 to FR-023** |
| **Math and Extended Markdown** | **FR-030 to FR-034** |
| **Dynamic Visualization and Navigation** | **FR-060 to FR-072** |
| **Workflow and Operational Ergonomics** | **FR-090 to FR-095** |
| **LLM Integration** | **FR-040 to FR-045** |
| **GraphRAG** | **FR-050 to FR-054** |
| **Templates System** | **FR-080 to FR-085** |


## **8.3 Architecture Decision Log**

| **Decision** | **Chosen** | **Alternatives Considered** | **Rationale** |
| - | - | - | - |
| **App Framework** | **Tauri 2.0** | **Electron, Flutter, Native (Qt/GTK)** | **Smallest binary, native Rust backend, Android support via Tauri Mobile, avoids 150MB+ Electron runtime** |
| **UI Layer** | **React 18 + TypeScript** | **Leptos (Rust/WASM), Yew, Blazor** | **Fastest iteration speed for a solo Rust-learning developer; CodeMirror 6 is first-class in React, no JS bridge needed** |
| **Database** | **SurrealDB (native, SurrealKV)** | **SurrealDB (WASM/IndxDB), SQLite + graph layer** | **No WASM embedding gotchas (ring, time shims, transaction lifetime); native performance; graph + vector in one engine** |
| **Editor** | **CodeMirror 6** | **Monaco, ProseMirror, Lexical** | **Best-in-class extensibility, Markdown and language-pack ecosystem, actively maintained** |
| **Graph Viz** | **D3.js or Sigma.js** | **Cytoscape.js, VisNetwork** | **D3 has the richest force-directed graph ecosystem; Sigma.js offers better performance for large graphs** |
| **Build Tool (Frontend)** | **Vite** | **Webpack, Parcel** | **Fastest HMR in development; default for Tauri 2.0 React template** |
