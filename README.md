# detect-lang

[![Build Status](https://travis-ci.org/vallentin/detect-lang.svg?branch=master)](https://travis-ci.org/vallentin/detect-lang)
[![Latest Version](https://img.shields.io/crates/v/detect-lang.svg)](https://crates.io/crates/detect-lang)
[![Docs](https://docs.rs/detect-lang/badge.svg)](https://docs.rs/detect-lang)
[![License](https://img.shields.io/github/license/vallentin/detect-lang.svg)](https://github.com/vallentin/detect-lang)

This crate is a utility for identifying names of programming languages (and related files) from paths and file extensions.

This is **not** a crate for detecting **natural** languages.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
detect-lang = "0.1"
```

## Releases

Release notes are available in the repo at [CHANGELOG.md].

[CHANGELOG.md]: CHANGELOG.md

## Paths and Extensions

Languages can be identified from paths using [`from_path`]
or directly from extensions using [`from_extension`].

[`from_path`]: https://docs.rs/detect-lang/*/detect_lang/fn.from_path.html
[`from_extension`]: https://docs.rs/detect-lang/*/detect_lang/fn.from_extension.html

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

In short, the language [`id`] is a lowercase version of [`name`].
However, it also replaces symbols making it usable as a [URL slug].

For instance `foo.hpp` is identified as language name `C++` and
language ID `cpp`.

[`id`]: https://docs.rs/detect-lang/*/detect_lang/struct.Language.html#method.id
[`name`]: https://docs.rs/detect-lang/*/detect_lang/struct.Language.html#method.name
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

## Always Lowercase

If the extension is guaranteed to always be lowercase,
then consider using [`from_lowercase_extension`] to avoid
allocation and conversion to lowercase.

[`from_lowercase_extension`]: https://docs.rs/detect-lang/*/detect_lang/fn.from_lowercase_extension.html

```rust
use detect_lang::{from_extension, from_lowercase_extension, Language};

assert_eq!(from_lowercase_extension("json"), Some(Language("JSON", "json")));
assert_eq!(from_lowercase_extension("jSoN"), None);

assert_eq!(from_extension("json"), Some(Language("JSON", "json")));
assert_eq!(from_extension("jSoN"), Some(Language("JSON", "json")));
```

## Match Example

```rust
use std::path::Path;
use detect_lang::{from_path, Language};

let path = Path::new("foo.rs");
match from_path(path) {
    //   Language(name, id)
    Some(Language(_, "rust")) => println!("This is Rust"),
    Some(Language(..))        => println!("Well it's not Rust"),
    None                      => println!("Ehh, what?"),
}
```
