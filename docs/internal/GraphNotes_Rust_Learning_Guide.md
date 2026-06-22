# Rust Learning Guide for GraphNotes

**Tailored for:** Python + Go background · Solo developer building GraphNotes with Tauri 2.0
**Approach:** Every concept is explained three ways — as a Python bridge, a Go bridge, and as something you'll actually write in GraphNotes.

---

## How to Use This Guide

Read it **in order**. Each concept builds on the previous. The learning sequence is deliberate — you can't understand borrowing without ownership, and you can't understand async without iterators. Don't skip ahead; Rust's compiler errors stop making sense when you're missing a prerequisite.

**Suggested pace:** ~1 concept per day, spending time writing small programs in the [Rust Playground](https://play.rust-lang.org) before moving on.

---

## Learning Sequence at a Glance

```
1. Variables & Mutability        ← the simplest Rust surprise
2. Ownership                     ← Rust's core idea; nothing else makes sense without this
3. Borrowing & References        ← how you share data without transferring ownership
4. Structs & Enums               ← how Rust models data
5. Pattern Matching              ← Rust's superpower for working with enums
6. Error Handling (Result/Option) ← replaces try/except and if err != nil
7. Traits                        ← Rust's interface system; needed for everything after this
8. Generics                      ← write one function that works for many types
9. Closures & Iterators          ← Rust's functional toolkit; replaces most for loops
10. Lifetimes                    ← the last hard concept; often avoidable early on
11. Async / Await (Tokio)        ← how GraphNotes does I/O without blocking
12. Modules & Crates             ← organizing a real multi-file project
13. Tauri Commands               ← where everything comes together for GraphNotes
```

---

## 1. Variables and Mutability

### The concept
In Rust, all variables are **immutable by default**. You must explicitly opt into mutability.

### Python bridge
```python
# Python: everything is mutable by default
x = 5
x = 10  # fine
```
```rust
// Rust: immutable by default — this FAILS to compile
let x = 5;
x = 10;  // ❌ error: cannot assign twice to immutable variable

// You must ask for mutability
let mut x = 5;
x = 10;  // ✅
```

### Go bridge
```go
// Go: variables are mutable by default, similar to Python
x := 5
x = 10  // fine
```
Rust is stricter here than both. The reason: the compiler uses mutability to reason about data races and aliasing. It's not just style — it's a correctness guarantee.

### GraphNotes example
In your Tauri command handlers, most parameters will be `let` bindings you only read. When you build up a vector of notes during vault scanning, that's when you need `mut`:

```rust
#[tauri::command]
async fn note_list(state: tauri::State<'_, AppState>) -> Result<Vec<NoteSummary>, String> {
    let mut notes = Vec::new();   // mut because we'll push into it
    // ... fill notes from DB ...
    Ok(notes)
}
```

### Key rule
Default to `let`. Only add `mut` when the compiler tells you something needs to change, or when the logic clearly requires mutation (building a collection, incrementing a counter).

---

## 2. Ownership

### The concept
Ownership is Rust's most important idea. Every value has exactly one owner. When the owner goes out of scope, the value is dropped (memory freed). **There is no garbage collector.**

### Python bridge
```python
# Python: reference counting under the hood, automatic GC
x = [1, 2, 3]
y = x          # both x and y point to the same list
y.append(4)
print(x)       # [1, 2, 3, 4] — they share the same object
```
```rust
// Rust: moving, not sharing
let x = vec![1, 2, 3];
let y = x;          // x is MOVED into y
println!("{:?}", x); // ❌ error: x was moved
println!("{:?}", y); // ✅
```

### Go bridge
```go
// Go: slices are reference types, sharing is implicit
x := []int{1, 2, 3}
y := x          // y shares the underlying array
y = append(y, 4)
fmt.Println(x)  // might be [1, 2, 3, 4] depending on capacity — subtle!
```
Go's slice sharing is actually a common source of bugs. Rust makes ownership explicit to prevent exactly this class of problem.

### The move, copy, and clone model
- **Move:** non-trivial types (`String`, `Vec`, structs) are moved — the original is invalidated
- **Copy:** primitive types (`i32`, `bool`, `f64`, `char`) are copied — both bindings remain valid
- **Clone:** explicitly duplicate data with `.clone()` when you need two independent copies

```rust
let a = 5;
let b = a;       // i32 is Copy — a is still valid
println!("{}", a); // ✅

let s1 = String::from("vault");
let s2 = s1.clone();   // explicit deep copy
println!("{}", s1);    // ✅ s1 is still valid
```

### GraphNotes example
Your `NoteRecord` struct will be moved around a lot. When you insert a note into SurrealDB and also want to return it to the frontend, you'll clone:

```rust
let note = build_note_record(&path, &content);
db.create::<Option<NoteRecord>>("note")
    .content(note.clone())   // clone here so we can still use note below
    .await?;
Ok(note)   // move note into the return value
```

### Mental model
Think of ownership like a safety deposit box key. One person holds the key at a time. You can hand it off (move), or make a copy of the contents (clone) for someone else. You can also let someone look inside without handing over the key — that's borrowing (next section).

---

## 3. Borrowing and References

### The concept
Instead of moving a value, you can **lend** it with a reference (`&`). The original owner keeps ownership. Rust enforces two rules at compile time:
1. You can have **many immutable references** (`&T`) simultaneously, OR
2. You can have **exactly one mutable reference** (`&mut T`) — but not both at once

### Python bridge
```python
# Python: everything is implicitly passed by reference
def process(items):
    items.append("new")   # mutates the original

data = ["a", "b"]
process(data)
print(data)  # ["a", "b", "new"] — original mutated
```
Python doesn't distinguish between "reading" and "writing" access to a value. Rust forces you to be explicit.

### Go bridge
```go
// Go: pointers are explicit, but no borrow checker
func process(items *[]string) {
    *items = append(*items, "new")
}
data := []string{"a", "b"}
process(&data)
```
Go has pointers and the compiler won't stop you from having multiple mutable pointers to the same data across goroutines. Rust's borrow checker prevents data races at compile time.

### References in Rust
```rust
fn print_length(s: &String) {  // borrows — s is a reference
    println!("length: {}", s.len());
    // s is not owned here; we can't move it or drop it
}

let vault_name = String::from("my-vault");
print_length(&vault_name);          // lend it
println!("still here: {}", vault_name);  // ✅ still valid
```

### The borrow checker rule in plain English
- **Many readers:** you can share a reference with multiple things at once, as long as nobody is writing
- **One writer:** if someone has a mutable reference, nobody else can have any reference at all

```rust
let mut note_content = String::from("# Hello");

let r1 = &note_content;        // immutable borrow
let r2 = &note_content;        // another immutable borrow — fine
println!("{} {}", r1, r2);     // ✅ — both reads, no conflict

let r3 = &mut note_content;    // mutable borrow
// println!("{}", r1);         // ❌ — can't mix immutable and mutable borrows
```

### GraphNotes example
Almost every Tauri command takes references to `AppState` values:

```rust
#[tauri::command]
async fn note_read(
    path: String,                          // owned — caller gave us this string
    state: tauri::State<'_, AppState>,     // &AppState under the hood — we borrow it
) -> Result<String, String> {
    let db = &state.db;   // borrow the DB handle, don't move it
    // ... query DB ...
    Ok(content)
}
```

### When the borrow checker rejects your code
The borrow checker is not the enemy — it's telling you that your code has a potential aliasing or lifetime bug. Common solutions:
- Clone the value if you need two independent owners
- Restructure to avoid holding a reference while mutating
- Use `Arc<Mutex<T>>` or `Arc<RwLock<T>>` for shared mutable state across async tasks (you'll use this in `AppState`)

---

## 4. Structs and Enums

### The concept
Rust uses `struct` for product types (data with multiple named fields) and `enum` for sum types (a value that is one of several variants, each potentially carrying different data). Enums in Rust are far more powerful than in Python or Go.

### Structs

**Python bridge:**
```python
@dataclass
class NoteRecord:
    path: str
    title: str
    content: str
    created_at: datetime
```

**Go bridge:**
```go
type NoteRecord struct {
    Path      string
    Title     string
    Content   string
    CreatedAt time.Time
}
```

**Rust:**
```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteRecord {
    pub path: String,
    pub title: String,
    pub content: String,
    pub created_at: String,  // ISO 8601 string for SurrealDB / JSON compatibility
}
```

The `#[derive(...)]` line auto-implements traits (think interfaces): `Debug` for printing, `Clone` for `.clone()`, `Serialize`/`Deserialize` for JSON — which Tauri uses to send your structs over IPC.

### Enums

Python and Go have enums, but Rust's are **algebraic data types** — each variant can carry different data:

**Python (plain enum):**
```python
from enum import Enum
class LinkType(Enum):
    PLAIN = 1
    ALIASED = 2
    SECTION = 3
```
Python enums carry at most a single value per variant.

**Go (no sum types — typically int constants):**
```go
type LinkType int
const (
    Plain   LinkType = iota
    Aliased
    Section
)
```
Go has no native way to attach different data to each variant.

**Rust (sum type with data per variant):**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub enum WikiLink {
    Plain { target: String },
    Aliased { target: String, alias: String },
    Section { target: String, section: String },
    BlockRef { target: String, block_id: String },
    Transclusion { target: String },
}
```

Each variant of `WikiLink` carries different data. The compiler enforces that you handle all variants whenever you use a `WikiLink` value. This is huge for correctness in a parser.

### GraphNotes example
Your wikilink parser will return `WikiLink` variants and your indexer will match on them:

```rust
fn create_edge(link: WikiLink, source_id: &str, db: &Surreal<Any>) {
    match link {
        WikiLink::Plain { target } => { /* create basic edge */ }
        WikiLink::Aliased { target, alias } => { /* create edge with alias field */ }
        WikiLink::Section { target, section } => { /* create edge with section_anchor field */ }
        WikiLink::BlockRef { target, block_id } => { /* create edge with block_id field */ }
        WikiLink::Transclusion { target } => { /* create transclusion edge */ }
    }
}
```

---

## 5. Pattern Matching

### The concept
`match` is Rust's control flow for enums (and much more). It's exhaustive — you must handle every variant or the code won't compile. Think of it as a `switch` that the compiler verifies is complete.

### Python bridge
```python
# Python: match (3.10+) or if/elif chains
match link_type:
    case "plain":   handle_plain()
    case "aliased": handle_aliased()
    # forgetting a case is a runtime bug — Python won't warn you
```

### Go bridge
```go
// Go: switch — but not exhaustive
switch linkType {
case "plain":   handlePlain()
case "aliased": handleAliased()
default:        panic("unhandled")  // you have to manually add this
}
```

### Rust
```rust
match link {
    WikiLink::Plain { target } => handle_plain(target),
    WikiLink::Aliased { target, alias } => handle_aliased(target, alias),
    WikiLink::Section { target, section } => handle_section(target, section),
    WikiLink::BlockRef { target, block_id } => handle_block_ref(target, block_id),
    WikiLink::Transclusion { target } => handle_transclusion(target),
    // If you forget a variant ↑ the compiler errors. You can't ship a bug here.
}
```

### `if let` — match one variant, ignore the rest
```rust
// Full match when you only care about one case is verbose:
if let WikiLink::Plain { target } = link {
    println!("plain link to {}", target);
}
```

### `while let` — pull from a channel until it closes
```rust
while let Some(event) = file_watcher_rx.recv().await {
    process_file_event(event).await;
}
```

### GraphNotes example
Your error handling (next section) uses `match` constantly. So does your IPC layer — Tauri commands return `Result<T, E>`, and the frontend gets either a success value or an error string depending on which variant the Rust code produced.

---

## 6. Error Handling — `Result` and `Option`

### The concept
Rust has no exceptions. Instead, functions that can fail return `Result<T, E>`: either a success value (`Ok(T)`) or an error (`Err(E)`). Functions that might return nothing return `Option<T>`: either `Some(T)` or `None`.

### Python bridge
```python
# Python: exceptions propagate implicitly
try:
    content = read_file(path)
    parsed = parse_markdown(content)
except FileNotFoundError as e:
    handle_error(e)
```
In Python, any function might throw — you can't tell from the signature. In Rust, if a function can fail, it must say so in its return type.

### Go bridge
```go
// Go: multiple return values for errors — Rust's inspiration
content, err := readFile(path)
if err != nil {
    return err
}
parsed, err := parseMarkdown(content)
if err != nil {
    return err
}
```
Rust is similar to Go here, but with a more ergonomic propagation operator.

### `Result` in Rust
```rust
fn read_note(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?;   // the ? operator
    Ok(content)
}
```

**The `?` operator** is Rust's answer to Go's `if err != nil { return err }`. It:
1. If `Ok(value)` → unwraps and continues
2. If `Err(e)` → returns `Err(e)` from the current function immediately

So `?` replaces approximately 3 lines of Go error handling with 1 character.

### `Option` in Rust
```rust
// Looking up a note by title might find nothing
fn find_note_by_title(title: &str, notes: &[NoteRecord]) -> Option<&NoteRecord> {
    notes.iter().find(|n| n.title == title)
}

// Use it:
match find_note_by_title("My Note", &vault_notes) {
    Some(note) => println!("Found: {}", note.path),
    None       => println!("Note not found"),
}
```

**Python bridge:** `Option<T>` replaces `None` checks. In Python, any value might be `None` at runtime. In Rust, if a type is `T` (not `Option<T>`), the compiler guarantees it is never `None`.

**Go bridge:** `Option<T>` replaces the `(value, bool)` pattern Go uses for map lookups and optional returns.

### Error handling in Tauri commands
Tauri commands must return `Result<T, String>` (the error must be serializable). Use `map_err` to convert your internal error types:

```rust
#[tauri::command]
async fn note_read(path: String) -> Result<String, String> {
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;
    Ok(content)
}
```

### The `?` chain — reading a note and querying the DB
```rust
async fn index_note(path: &str, db: &Surreal<Any>) -> Result<(), AppError> {
    let content = tokio::fs::read_to_string(path).await?;   // io::Error → AppError
    let (frontmatter, body) = parse_frontmatter(&content)?; // ParseError → AppError
    let links = extract_wikilinks(&body)?;
    db.create("note").content(/* ... */).await?;            // surrealdb::Error → AppError
    Ok(())
}
```

Each `?` propagates the error up the call stack, just like Go's `return err` — but in one character and with type safety.

---

## 7. Traits

### The concept
Traits define shared behaviour — like interfaces in Go or abstract base classes in Python. A type implements a trait by providing the required methods. Unlike Go interfaces, Rust trait implementations are explicit.

### Python bridge
```python
# Python: duck typing — if it has the method, it works
class Indexable:
    def index(self, db): ...

class Note(Indexable):
    def index(self, db):
        db.insert(self)
```
Python uses implicit duck typing. Any object with the right methods qualifies.

### Go bridge
```go
// Go: implicit interface satisfaction — closest to Rust traits
type Indexable interface {
    Index(db Database) error
}

type Note struct { ... }
func (n Note) Index(db Database) error { ... }
// Note implicitly satisfies Indexable
```
Rust is similar to Go but requires **explicit** `impl Trait for Type`.

### Rust
```rust
pub trait Indexable {
    fn index(&self, db: &Surreal<Any>) -> impl Future<Output = Result<(), AppError>>;
}

impl Indexable for NoteRecord {
    async fn index(&self, db: &Surreal<Any>) -> Result<(), AppError> {
        db.create("note").content(self).await?;
        Ok(())
    }
}
```

### The most important derived traits for GraphNotes

You'll use these constantly via `#[derive(...)]`:

| Trait | What it does | GraphNotes use |
|---|---|---|
| `Debug` | Enables `{:?}` printing | Debugging structs in logs |
| `Clone` | Enables `.clone()` | Duplicating data before moving |
| `Serialize` | JSON serialization (serde) | Sending data over Tauri IPC |
| `Deserialize` | JSON deserialization (serde) | Receiving data from Tauri IPC / SurrealDB |
| `PartialEq` | Enables `==` comparison | Testing equality in unit tests |
| `Default` | Provides a zero/empty value | `AppState::default()` |

### Trait objects (dynamic dispatch) — when the type isn't known at compile time
```rust
// &dyn Error means "any type that implements Error"
fn handle_error(e: &dyn std::error::Error) {
    eprintln!("Error: {}", e);
}
```
This is Rust's equivalent of Go's `interface{}` or Python's `Any` — but only for traits, not arbitrary types.

---

## 8. Generics

### The concept
Generics let you write one function or struct that works for multiple types, with the compiler generating specialised code for each use. Unlike Python (where everything is typed dynamically) and more like Go generics (1.18+).

### Python bridge
```python
# Python: implicit generics — works for any type, no declaration needed
def first(items):      # works for list of anything
    return items[0]
```
No generics syntax in Python — it's duck typing.

### Go bridge
```go
// Go 1.18+: explicit generics
func First[T any](items []T) T {
    return items[0]
}
```

### Rust
```rust
fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}
```

The `<T>` declares a generic type parameter. The `Option<&T>` return handles the empty slice case.

### Adding trait bounds — constraining what T can be
```rust
// T must implement Debug to allow printing
fn print_first<T: std::fmt::Debug>(items: &[T]) {
    if let Some(item) = items.first() {
        println!("{:?}", item);
    }
}
```
The `: Debug` part is a **trait bound** — you constrain `T` to only types that implement `Debug`. Go uses similar constraint syntax.

### GraphNotes example
You won't write much custom generic code early on — the Rust ecosystem handles it. But you'll **read** generics constantly:

```rust
// SurrealDB's query method signature (simplified)
async fn query<R: DeserializeOwned>(&self, sql: &str) -> Result<Vec<R>, Error>;

// When you call it, Rust infers T from the expected return type:
let notes: Vec<NoteRecord> = db.query("SELECT * FROM note").await?;
//         ^^^^^^^^^^^^^^^^^^^^^ Rust infers R = NoteRecord
```

---

## 9. Closures and Iterators

### The concept
Closures are anonymous functions that can capture variables from their surrounding scope. Iterators are lazy chains of operations over collections. Together, they replace most `for` loops with more expressive, composable code.

### Python bridge
```python
# Python: list comprehensions and lambdas
paths = [note["path"] for note in notes if note["title"].startswith("2026")]
# or:
paths = list(map(lambda n: n["path"], filter(lambda n: n["title"].startswith("2026"), notes)))
```

### Go bridge
```go
// Go: manual loops — no native map/filter/collect
var paths []string
for _, note := range notes {
    if strings.HasPrefix(note.Title, "2026") {
        paths = append(paths, note.Path)
    }
}
```
Go has no built-in map/filter on slices (pre-generics, still verbose). Rust makes this ergonomic.

### Rust iterators
```rust
let paths: Vec<String> = notes
    .iter()
    .filter(|note| note.title.starts_with("2026"))
    .map(|note| note.path.clone())
    .collect();
```

This is **lazy** — no intermediate `Vec` is allocated for the filter step. The chain only allocates the final `collect()` result.

### Common iterator methods you'll use in GraphNotes

| Method | What it does | Python equivalent |
|---|---|---|
| `.iter()` | Borrow elements as `&T` | `iter()` |
| `.into_iter()` | Consume, yields owned `T` | — |
| `.filter(|x| ...)` | Keep matching elements | `filter()` |
| `.map(|x| ...)` | Transform each element | `map()` |
| `.collect::<Vec<_>>()` | Materialize into a `Vec` | `list()` |
| `.find(|x| ...)` | First match | `next(filter(...))` |
| `.any(|x| ...)` | True if any match | `any()` |
| `.all(|x| ...)` | True if all match | `all()` |
| `.flat_map(|x| ...)` | Map then flatten | `chain(map(...))` |
| `.enumerate()` | Pair each element with its index | `enumerate()` |
| `.for_each(|x| ...)` | Side-effectful loop | `for x in ...` |

### Closures capturing their environment
```rust
let query_prefix = "2026";   // captured by the closure
let matching: Vec<&NoteRecord> = notes
    .iter()
    .filter(|note| note.title.starts_with(query_prefix))  // query_prefix captured here
    .collect();
```

### GraphNotes example — extracting all wikilinks from multiple notes
```rust
let all_links: Vec<WikiLink> = notes
    .iter()
    .flat_map(|note| extract_wikilinks(&note.content))  // one Vec<WikiLink> per note → flatten into one
    .collect();
```

---

## 10. Lifetimes

### The concept
Lifetimes are Rust's way of proving at compile time that references don't outlive the data they point to. The compiler infers most lifetimes automatically — you only need to write them when the compiler can't figure it out.

### The key rule
A reference cannot outlive the data it refers to.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

The `'a` annotation means: "the returned reference lives at least as long as both inputs." Without this, the compiler can't verify the return value doesn't outlive either input.

### Python bridge
Python has no lifetime concept — the GC handles object lifetimes. Dangling references are impossible (but unbounded memory growth is possible).

### Go bridge
Go uses GC — pointers are valid as long as anything references the object. No lifetime annotations.

### When you'll encounter lifetimes in GraphNotes
**Mostly, you won't.** The compiler infers lifetimes in the majority of cases. You'll encounter explicit lifetimes in:
- Structs that hold references (uncommon in Tauri commands, where you typically own data)
- The `tauri::State<'_, AppState>` parameter — the `'_` is an inferred lifetime; it means "the state reference lives as long as this function call"

**Practical rule for early GraphNotes development:** if the compiler asks you to add a lifetime, and you're unsure, try cloning the relevant data instead. Own your data in structs rather than holding references to external data. This avoids lifetimes entirely for most of your code.

---

## 11. Async / Await (Tokio)

### The concept
Rust's `async`/`await` is similar to Python's `asyncio` and similar in syntax to JavaScript/TypeScript. The key difference: you must choose an **async runtime**. For GraphNotes, Tauri uses **Tokio** — so all your async code runs on Tokio's thread pool.

### Python bridge
```python
import asyncio

async def read_notes(path):
    content = await asyncio.to_thread(open(path).read)
    return content

asyncio.run(read_notes("vault/note.md"))
```

### Go bridge
```go
// Go: goroutines + channels instead of async/await
func readNotes(path string) chan string {
    ch := make(chan string)
    go func() {
        content, _ := os.ReadFile(path)
        ch <- string(content)
    }()
    return ch
}
```
Go uses goroutines and channels rather than async/await. Rust's model is more similar to Python's.

### Rust + Tokio
```rust
use tokio::fs;

async fn read_note(path: &str) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(path).await?;
    Ok(content)
}

#[tokio::main]
async fn main() {
    let content = read_note("vault/note.md").await.unwrap();
    println!("{}", content);
}
```

### The `await` keyword
`.await` suspends the current async function until the future resolves, yielding control to the Tokio runtime to run other tasks. This is non-blocking — while one task waits for disk I/O, another can run.

### Spawning background tasks
```rust
// Spawn a background task (fire-and-forget from the command handler's perspective)
tokio::spawn(async move {
    // This runs concurrently with the calling code
    index_vault(vault_path, db).await;
});
```
**`move` in closures:** The `async move` means the closure takes ownership of the variables it captures (`vault_path`, `db`). This is almost always what you want for spawned tasks.

### `Arc<Mutex<T>>` — sharing state across async tasks
Since multiple Tokio tasks may access `AppState` concurrently, mutable state needs protection:

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub db: Arc<Surreal<Any>>,              // SurrealDB is already thread-safe
    pub vault_path: Arc<RwLock<Option<PathBuf>>>,  // needs RwLock for mutable access
}
```

- `Arc<T>` — Atomically Reference Counted: multiple owners, any thread
- `Mutex<T>` / `RwLock<T>` — exclusive write access, shared read access

**Python bridge:** `Arc<RwLock<T>>` ≈ a `threading.Lock()` around a value, but the compiler forces you to acquire the lock before touching the data.

**Go bridge:** `Arc<RwLock<T>>` ≈ `sync.RWMutex` + the value it protects, but as a single unit that can't be accidentally accessed without locking.

### GraphNotes example — streaming LLM tokens
```rust
#[tauri::command]
async fn llm_chat_send(
    message: String,
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let client = state.llm_client.clone();
    let handle = app.clone();

    tokio::spawn(async move {
        let mut stream = client.stream_chat(&message).await.unwrap();
        while let Some(token) = stream.next().await {
            handle.emit("llm_token", token).ok();
        }
        handle.emit("llm_done", ()).ok();
    });

    Ok(())  // returns immediately; tokens arrive via events
}
```

---

## 12. Modules and Crates

### The concept
Rust organizes code into **modules** (within a crate, like packages within a Python file or Go package) and **crates** (compiled units, like Python packages or Go modules).

### Python bridge
```python
# Python: file = module, directory with __init__.py = package
from engine.parser import extract_wikilinks
from db.notes import upsert_note
```

### Go bridge
```go
// Go: directory = package
import (
    "graphnotes/engine/parser"
    "graphnotes/db/notes"
)
```

### Rust module system
```rust
// src-tauri/src/main.rs
mod commands;    // loads src-tauri/src/commands/mod.rs or src-tauri/src/commands.rs
mod engine;
mod db;
mod llm;
mod state;

// src-tauri/src/commands/mod.rs
pub mod vault;
pub mod notes;
pub mod graph;
pub mod llm;

// src-tauri/src/commands/vault.rs
pub use crate::state::AppState;  // bring AppState into scope from root
use crate::engine::indexer::index_vault;  // use indexer from engine module
```

### Visibility
- `pub` — public to other modules and crates
- `pub(crate)` — public within this crate only (often the right choice for internal APIs)
- no prefix — private to this module (default)

### `Cargo.toml` — adding dependencies
```toml
[dependencies]
tauri = { version = "2", features = ["macos-private-api"] }
surrealdb = { version = "2", features = ["kv-surrealkv"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
pulldown-cmark = "0.12"
async-openai = "0.27"
notify = "7"
```

**Python bridge:** `Cargo.toml` ≈ `pyproject.toml` / `requirements.txt`
**Go bridge:** `Cargo.toml` ≈ `go.mod`

---

## 13. Tauri Commands — Where It All Comes Together

This is the pattern you'll write dozens of times in GraphNotes. Every Tauri command uses almost every concept from this guide.

### A complete Tauri command — note_save

```rust
// src-tauri/src/commands/notes.rs

use serde::{Serialize, Deserialize};         // Trait derives (concept 7)
use tauri::State;                             // Generics (concept 8)
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize)]      // derive macros auto-implement traits
pub struct SaveNoteRequest {
    pub path: String,                         // owned String (concept 2)
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct SaveNoteResponse {
    pub success: bool,
    pub updated_at: String,
}

// async fn (concept 11) + Result return (concept 6)
#[tauri::command]
pub async fn note_save(
    request: SaveNoteRequest,                 // owned, moved in (concept 2)
    state: State<'_, AppState>,               // borrowed state (concept 3, lifetime 10)
) -> Result<SaveNoteResponse, String> {       // Result for error handling (concept 6)

    // Write to disk (async I/O, concept 11)
    tokio::fs::write(&request.path, &request.content)   // borrows path and content (concept 3)
        .await
        .map_err(|e| format!("Write failed: {e}"))?;    // ? propagates error (concept 6)

    // Re-parse and upsert into SurrealDB
    let (frontmatter, _body) = crate::engine::parser::parse_frontmatter(&request.content)
        .map_err(|e| e.to_string())?;

    let updated_at = chrono::Utc::now().to_rfc3339();

    // Struct literal (concept 4)
    let record = crate::db::NoteRecord {
        path: request.path.clone(),           // clone because we need it below too (concept 2)
        content: request.content,             // moved — last use of request.content (concept 2)
        frontmatter,
        updated_at: updated_at.clone(),
    };

    state.db
        .create::<Option<crate::db::NoteRecord>>("note")
        .content(record)
        .await
        .map_err(|e| e.to_string())?;

    Ok(SaveNoteResponse {
        success: true,
        updated_at,
    })
}
```

And the TypeScript side — notice how clean the call site is:

```typescript
// src/hooks/useNote.ts
import { invoke } from "@tauri-apps/api/core";

interface SaveNoteResponse {
  success: boolean;
  updated_at: string;
}

async function saveNote(path: string, content: string): Promise<SaveNoteResponse> {
  return invoke<SaveNoteResponse>("note_save", {
    request: { path, content }
  });
}
```

---

## Recommended Practice Path

1. **Week 1 — Concepts 1–5:** Work through [The Rust Book](https://doc.rust-lang.org/book/) chapters 1–6. Write small programs: a struct that parses a fake frontmatter string, an enum for wikilink types.

2. **Week 2 — Concepts 6–9:** Rust Book chapters 7–13. Build a small CLI tool in Rust that reads a directory of `.md` files and prints their titles. Use `Result`/`Option` for all error handling. Use iterators over `filter`/`map`/`collect`.

3. **Week 3 — Concepts 10–12:** Skim Rust Book chapter 10 (lifetimes). Read the [Tokio tutorial](https://tokio.rs/tokio/tutorial). Build an async file reader that processes multiple files concurrently with `tokio::spawn`.

4. **Week 4 — Concept 13 + Phase 0:** Build Phase 0 of GraphNotes. Every time you're confused, come back to this guide and identify which concept is blocking you.

5. **Ongoing — learn concepts as you hit them:** Rust's compiler errors are exceptional teachers. When you get a borrow checker error, the message usually tells you exactly what concept to revisit. Trust the compiler — it's not wrong.

---

## Quick Reference: Python/Go → Rust Cheat Sheet

| Python / Go | Rust | Notes |
|---|---|---|
| `x = 5` | `let x = 5;` | Immutable by default |
| `x = 5; x = 10` | `let mut x = 5; x = 10;` | `mut` required |
| `def f(x):` | `fn f(x: String) -> String` | Types required |
| `x = y` (list/dict) | `let x = y.clone();` | Explicit copy of heap data |
| `None` | `Option::None` | No null; use `Option<T>` |
| `raise ValueError("msg")` | `return Err("msg".into())` | No exceptions |
| `try/except` | `match result { Ok(v) => ..., Err(e) => ... }` or `?` | |
| `await coro()` | `function().await` | Same concept, different syntax |
| `asyncio.create_task()` | `tokio::spawn(async { ... })` | Background task |
| `threading.Lock()` | `Arc<Mutex<T>>` | Shared mutable state |
| `@dataclass` | `#[derive(Debug, Clone, ...)] struct` | |
| `class Foo(Bar):` | `impl Bar for Foo { ... }` | Explicit trait impl |
| `from module import X` | `use crate::module::X;` | |
| `go func()` | `tokio::spawn(async move { ... })` | Concurrent task |
| `err != nil { return err }` | `?` | Error propagation |
| `if x is None:` | `if x.is_none():` or `match x { None => ... }` | |
| `x if x is not None else default` | `x.unwrap_or(default)` | |
