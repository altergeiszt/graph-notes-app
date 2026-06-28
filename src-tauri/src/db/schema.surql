-- ── Notes ────────────────────────────────────────────────────────────
DEFINE TABLE IF NOT EXISTS note SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS path         ON note TYPE string;
DEFINE FIELD IF NOT EXISTS title        ON note TYPE string;
DEFINE FIELD IF NOT EXISTS content      ON note TYPE string;
DEFINE FIELD IF NOT EXISTS frontmatter  ON note TYPE any;
DEFINE FIELD IF NOT EXISTS created_at   ON note TYPE datetime;
DEFINE FIELD IF NOT EXISTS updated_at   ON note TYPE datetime;
DEFINE INDEX IF NOT EXISTS note_path    ON note COLUMNS path UNIQUE;

-- ── Tags ─────────────────────────────────────────────────────────────
DEFINE TABLE IF NOT EXISTS tag SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS name  ON tag TYPE string;
DEFINE FIELD IF NOT EXISTS slug  ON tag TYPE string;
DEFINE INDEX IF NOT EXISTS tag_slug ON tag COLUMNS slug UNIQUE;

-- ── Dangling link placeholders ───────────────────────────────────────
DEFINE TABLE IF NOT EXISTS dangling_node SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS name ON dangling_node TYPE string;
DEFINE INDEX IF NOT EXISTS dangling_name ON dangling_node COLUMNS name UNIQUE;

-- ── Relations ────────────────────────────────────────────────────────
DEFINE TABLE IF NOT EXISTS links_to SCHEMAFULL TYPE RELATION IN note OUT note | dangling_node;
DEFINE FIELD IF NOT EXISTS alias          ON links_to TYPE option<string>;
DEFINE FIELD IF NOT EXISTS section_anchor ON links_to TYPE option<string>;
DEFINE FIELD IF NOT EXISTS block_id       ON links_to TYPE option<string>;
DEFINE FIELD IF NOT EXISTS line_number    ON links_to TYPE int;

DEFINE TABLE IF NOT EXISTS tagged_with SCHEMAFULL TYPE RELATION IN note OUT tag;
DEFINE FIELD IF NOT EXISTS source ON tagged_with TYPE string;  -- 'inline' | 'frontmatter'

-- ── Vector chunks (Phase 8, defined early) ───────────────────────────
DEFINE TABLE IF NOT EXISTS embedding_chunk SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS note_id     ON embedding_chunk TYPE record<note>;
DEFINE FIELD IF NOT EXISTS chunk_index ON embedding_chunk TYPE int;
DEFINE FIELD IF NOT EXISTS text        ON embedding_chunk TYPE string;
DEFINE FIELD IF NOT EXISTS vector      ON embedding_chunk TYPE array<float>;
