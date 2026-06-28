/// Notes
DEFINE TABLE note SCHEMAFULL;
DEFINE FIELD path ON note TYPE string;
DEFINE FIELD title ON note TYPE string;
DEFINE FIELD contents ON note TYPE string;
DEFINE FIELD frontmatter ON note TYPE object;
DEFINE FIELD created_at ON note TYPE datetime;
DEFINE FIELD updated_at ON note TYPE datetime;
DEFINE INDEX note_path ON note COLUMNS path UNIQUE;

/// Tags
DEFINE TABLE tag SCHEMAFULL;
DEFINE FIELD name ON tag TYPE string;
DEFINE FIELD slug ON tag TYPE string;
DEFINE INDEX tag_slug ON tag COLUMNS slug UNIQUE;

/// Dangling Links placeholders
DEFINE TABLE dangling_link SCHEMAFULL;
DEFINE FIELD name ON dangling_link TYPE string;
DEFINE INDEX dangling_name ON dangling_link COLUMNS name UNIQUE;

/// Relationships
DEFINE TABLE links_to SCHEMAFULL TYPE RELATION IN note OUT note | dangling_link;
DEFINE FIELD alias ON links_to TYPE option<string>;
DEFINE FIELD section_anchor ON links_to TYPE option<string>;
DEFINE FIELD block_id ON links_to TYPE option<string>;
DEFINE FIELD line_number ON links_to TYPE int;

DEFINE TABLE tagged_with SCHEMAFULL TYPE RELATION IN note OUT tag;
DEFINE FIELD source ON tagged_with TYPE string;

/// Vector Chunks
DEFINE TABLE embedding_chunk SCHEMAFULL;
DEFINE FIELD note_id ON embedding_chunk TYPE record<note>;
DEFINE FIELD chunk_index ON embedding_chunk TYPE int;
DEFINE FIELD text ON embedding_chunk TYPE string;
DEFINE FIELD vector ON embedding_chunk TYPE array<float>;