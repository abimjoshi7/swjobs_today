# Macros, attributes, derives, and clap

Learning notes for this project. Prefer short lines over long paragraphs.

---

## Short top-level name (binary vs clap)

Two different “names” exist:

- **Executable name** — what the OS runs (`swjobs_today`, `target/debug/swjobs_today`)
- **Usage name in help** — what clap prints (`Usage: swjobs_today <COMMAND>`)

### Change help text only
- On the root `Parser` struct:

```rust
#[derive(Parser)]
#[command(name = "swj", version, about = "Software jobs for today")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
```

- Effect: `--help` says `swj`
- Effect: does **not** rename the real binary on disk
- `cargo run -- today` still invokes `swjobs_today`

### Change the real binary name
- In `Cargo.toml`:

```toml
[[bin]]
name = "swj"
path = "src/main.rs"
```

- Effect: artifact becomes `target/debug/swj`
- Effect: after install/PATH, you type `swj today`
- Package/crate name can stay `swjobs_today`

### Other options
- Shell alias: `alias swj=swjobs_today`
- Symlink / rename after install
- Keep full name in help; short alias for daily use

**Recommendation for learning:** keep binary `swjobs_today` for clarity; optional `#[command(name = "swj")]` later if help feels long.

---

## Vocabulary

| Term | Meaning |
|------|---------|
| **Attribute** | `#[...]` annotation on an item (struct, field, fn, …) |
| **Derive macro** | Attribute of the form `#[derive(Foo)]` that **generates impls** |
| **Attribute macro** | Custom `#[foo]` that rewrites/transforms the item |
| **Function-like macro** | `println!`, `vec!`, `matches!` — call with `!` |
| **Proc-macro** | Compile-time Rust code in another crate that emits tokens |
| **Built-in derive** | Compiler-known: `Debug`, `Clone`, `Copy`, `PartialEq`, … |
| **Custom derive** | From a crate (clap, serde, thiserror) via proc-macro |

---

## Attributes in general

- Syntax: `#[outer]` attaches to the next item
- Inner form: `#![crate_attr]` applies to the enclosing module/crate
- Attributes can carry paths and args: `#[command(subcommand)]`, `#[derive(Debug, Clone)]`

### Categories you’ll see
- **Built-in language/compiler:** `#[allow(...)]`, `#[cfg(...)]`, `#[test]`, `#[inline]`
- **Built-in derive:** `#[derive(Debug, Clone, PartialEq)]`
- **Crate-provided derive:** `#[derive(Parser)]`, `#[derive(Serialize)]`
- **Crate-provided attributes used with derives:** `#[command(...)]`, `#[serde(rename = "...")]`
- **Tooling:** `#[tokio::main]` (attribute macro)

Attributes alone do nothing magic unless:
- the compiler understands them, or
- a proc-macro crate registers/handles them

---

## Built-in derives (`Debug`, `Clone`, `PartialEq`, …)

### What they are
- Listed in the Rust reference as **built-in** derives
- Implemented inside rustc (not as a normal dependency crate you write)
- Still written with the same `#[derive(...)]` syntax

### What they generate (idea)
- `Debug` → `impl fmt::Debug for T` with a debug printer
- `Clone` → `impl Clone for T` (field-by-field clone)
- `Copy` → marker + requires all fields `Copy`
- `PartialEq` / `Eq` → equality by fields
- `Default` → `fn default()` from field defaults
- `Hash` → hash fields in order

### Rules / limits
- Only work when **all fields** also implement the trait (usually)
- Fail with clear rustc errors if a field can’t
- No custom field attributes like `#[debug(skip)]` unless you use a **third-party** debug crate
- No runtime reflection — code is generated at compile time

### Example

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct JobId(String);

let a = JobId("abc".into());
let b = a.clone();
assert_eq!(a, b);
println!("{a:?}"); // needs Debug
```

### Mental model
- Built-in derive = “write the obvious trait impl for me”
- Small, predictable, language-standard

---

## Custom derives (clap, serde, …)

### What they are
- Proc-macro crates (`clap_derive`, `serde_derive`, …)
- Activated by features / dependencies (`clap` + `features = ["derive"]`)
- Same surface syntax: `#[derive(Parser)]`
- Different engine: your code is parsed as tokens → new Rust source is emitted into the crate

### How they differ from `Debug` / `Clone`

| | Built-in (`Debug`) | Custom (`Parser`) |
|--|--------------------|-------------------|
| Who implements | rustc | external crate |
| Needs dependency | no | yes |
| Needs feature flag | no | often (`derive`) |
| Companion field attrs | rare / none | common (`#[command]`, `#[arg]`) |
| Generated API | std traits | crate traits (`Parser`, `Subcommand`) |
| Failure mode | “X doesn’t implement Debug” | clap/serde-specific errors |
| Scope | equality, print, copy | CLI parsing, JSON, errors, … |

### Companion attributes
- Custom derives often **look for** extra attributes on the same item/fields
- Example: clap’s `#[command(subcommand)]` is **not** a separate magic language feature
- It is metadata the `Parser` derive reads while generating code
- Without the derive, those attributes may error as “cannot find attribute” / unused depending on setup

### Example (your project shape)

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]                          // custom derive → impl Parser for Cli
#[command(name = "swjobs_today", version)] // companion attr → help/version metadata
pub struct Cli {
    #[command(subcommand)]                // companion attr → this field is a subcommand enum
    pub command: Commands,
}

#[derive(Subcommand)]                     // custom derive → impl Subcommand for Commands
pub enum Commands {
    /// Show today's ranked jobs           // doc comment → help text
    Today,
}
```

---

## Function-like macros vs derives

### Function-like (`foo!`)
- Invoked like a function with `!`
- Examples: `println!("hi")`, `vec![1, 2]`, `include_str!("x")`
- Can take varied syntax (not always valid Rust)
- Expand to expressions, items, or statements depending on the macro

### Derive macros (`#[derive(Foo)]`)
- Only attach to structs/enums/unions (mostly)
- Must generate `impl` blocks (conventionally)
- Cannot arbitrarily rewrite the struct definition itself (the item stays; impls are added)

### Attribute macros (`#[foo]`)
- Can transform the whole item (replace/rewrite)
- Example patterns: `#[tokio::main] async fn main() { ... }` becomes a sync `main` that starts a runtime
- Stronger / more invasive than derive

### Declarative macros (`macro_rules!`)
- Written in your crate with pattern → template rules
- No separate proc-macro crate needed
- Great for small DSLs; weaker for deep AST analysis than proc-macros

---

## Clap deep dive (this repo)

### Traits
- `Parser` — “parse argv into Self”
- `Subcommand` — “parse one subcommand word into an enum variant”
- `Args` — “parse a group of flags/positionals into a struct” (later)
- `ValueEnum` — “parse a flag value from an enum” (later)

### Root command
- The struct with `#[derive(Parser)]` is the **top-level** command
- Binary name default ≈ package/bin name
- Override display with `#[command(name = "...")]`

### Subcommands
- Enum with `#[derive(Subcommand)]`
- Wired in with a field marked `#[command(subcommand)]`
- Variant `Today` → CLI token `today`
- Doc comments on variants become help lines

### Typical expansion story (conceptual)
- You write the annotated structs/enums
- `clap_derive` generates roughly:
  - `impl Parser for Cli { fn parse() ... }`
  - builders that register subcommands/flags
  - mapping from `ArgMatches` → your types
- At runtime: `Cli::parse()` builds that CLI definition and parses env args

### Common attributes you’ll meet next

```rust
#[derive(Parser)]
#[command(name = "swjobs_today", version, about = "...", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ranked jobs for today
    Today {
        /// Max rows to print
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
    Profile {
        #[command(subcommand)]
        action: ProfileCmd,
    },
}

#[derive(Subcommand)]
enum ProfileCmd {
    Show,
    Set {
        #[arg(long)]
        skills: Option<String>,
    },
}
```

Notes:
- `#[arg(...)]` = flag/positional config for a field
- Nested `#[command(subcommand)]` = `profile show` style
- `short` / `long` → `-l` / `--limit`
- `default_value_t` → typed default

### Help / version
- `version` in `#[command(version)]` pulls crate version from Cargo by default (with right clap features / setup)
- `--help` and `-h` come from clap’s `help` feature (on by default)
- Missing subcommand usually prints help / usage error and exits

---

## Worked examples side by side

### A. Built-in only

```rust
#[derive(Debug, Clone, PartialEq)]
struct Profile {
    name: String,
    skills: Vec<String>,
}
```

- No deps
- Enables print/compare/clone
- No argv parsing

### B. Clap root + one subcommand (Phase 0)

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "swjobs_today", version, about = "Software jobs for today")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show today's ranked jobs
    Today,
}
```

```rust
mod cli;
use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    match Cli::parse().command {
        Commands::Today => println!("No jobs yet."),
    }
}
```

### C. Same types also use built-in derives

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(clap::ValueEnum)] // later: parse from --sort freshness|score
enum SortKey {
    Freshness,
    Score,
}
```

- Multiple `#[derive(...)]` stacks are fine (or one list)
- Built-in + custom can coexist on one type

### D. Serde contrast (when you hit JSON profile)

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Profile {
    #[serde(default)]
    skills: Vec<String>,
}
```

- Same derive *pattern* as clap
- Different companion attrs (`#[serde(...)]`)
- Different generated traits (`Serialize` / `Deserialize`)

---

## Debugging mental checklist

- Derive not found → missing crate or feature (`derive`)
- `Cli::parse` not found → `use clap::Parser` (trait in scope)
- Subcommand not recognized → missing `#[command(subcommand)]` on the field
- Unexpected help name → `#[command(name = ...)]` vs actual `[[bin]]` name
- Attribute ignored / error → attribute belongs to a derive that isn’t present
- “expected subcommand” → you ran binary with no verb; try `--help`

---

## What to practice next (lines)

- Run `cargo expand` (optional tool) later to *see* generated impls
- Add `about` + doc comments; inspect `--help`
- Add one `#[arg(long)]` on `Today` when you need `--limit`
- Keep domain types (`Job`, `Profile`) on built-in derives first
- Add serde derives only when saving JSON

---

## External references

- Rust book — macros: https://doc.rust-lang.org/book/ch20-05-macros.html
- Rust reference — attributes: https://doc.rust-lang.org/reference/attributes.html
- Rust reference — derives: https://doc.rust-lang.org/reference/attributes/derive.html
- Clap derive cookbook: https://docs.rs/clap/latest/clap/_derive/index.html
- Clap tutorial: https://docs.rs/clap/latest/clap/_tutorial/index.html
