
# detect-lang

[![Latest Version](https://img.shields.io/crates/v/detect-lang.svg)](https://crates.io/crates/detect-lang)
[![Docs](https://docs.rs/detect-lang/badge.svg)](https://docs.rs/detect_lang)
[![License](https://img.shields.io/github/license/vallentin/detect-lang.svg)](https://github.com/vallentin/detect-lang)

This crate is a utility for identifying names of programming languages (and related files) from paths and file extensions.

This is **not** a crate for detecting **natural** languages.

## Paths and Extensions

Languages can be identified from paths using [`from_path`]([fn.from_path.html](https://docs.rs/detect_lang/*/detect_lang/fn.from_path.html))
or directly from extensions using [`from_extension`]([fn.from_extension.html](https://docs.rs/detect_lang/*/detect_lang/fn.from_extension.html)).

```rust
use detect_lang::from_path;
assert_eq!(from_path("foo.rs").unwrap().name(), "Rust");
assert_eq!(from_path("foo.md").unwrap().name(), "Markdown");

use detect_lang::from_extension;
assert_eq!(from_extension("rs").unwrap().name(), "Rust");
assert_eq!(from_extension("md").unwrap().name(), "Markdown");

// The case is ignored
assert_eq!(from_path("foo.jSoN").unwrap().name(), "JSON");
assert_eq!(from_extension("jSoN").unwrap().name(), "JSON");
```

## Language ID

In short, the language [`id`](https://docs.rs/detect_lang/*/detect_lang/struct.Language.html#method.id)
is a lowercase version of [`name`](https://docs.rs/detect_lang/*/detect_lang/struct.Language.html#method.name).
However, it also replaces symbols making it usable as a [URL slug].

For instance `foo.hpp` is identified as language name `C++` and
language ID `cpp`.

[URL slug]: https://en.wikipedia.org/wiki/Clean_URL#Slug

```rust
use detect_lang::from_path;
assert_eq!(from_path("foo.rs").unwrap().id(), "rust");
assert_eq!(from_path("foo.cpp").unwrap().id(), "cpp");
assert_eq!(from_path("foo.hpp").unwrap().id(), "cpp");

use detect_lang::from_extension;
assert_eq!(from_extension("rs").unwrap().id(), "rust");
assert_eq!(from_extension("cpp").unwrap().id(), "cpp");
assert_eq!(from_extension("hpp").unwrap().id(), "cpp");

// The case is ignored
assert_eq!(from_path("foo.jSoN").unwrap().id(), "json");
assert_eq!(from_extension("jSoN").unwrap().id(), "json");
```

## Match Example

```rust
use std::path::Path;
use detect_lang::{from_path, Language};

let path = Path::new("foo.rs");
match from_path(path) {
    Some(Language(_, "rust")) => println!("This is Rust"),
    Some(Language(..))        => println!("Well it's not Rust"),
    None                      => println!("Ehh, what?"),
}
```
