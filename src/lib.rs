//! Utility for identifying names of programming languages (and related files) from paths and file extensions.
//!
//! # Paths and Extensions
//!
//! Languages can be identified from paths using [`from_path`]
//! or directly from extensions using [`from_extension`].
//!
//! [`from_path`]: fn.from_path.html
//! [`from_extension`]: fn.from_extension.html
//!
//! ```
//! use detect_lang::from_path;
//! assert_eq!(from_path("foo.rs").unwrap().name(), "Rust");
//! assert_eq!(from_path("foo.md").unwrap().name(), "Markdown");
//!
//! use detect_lang::from_extension;
//! assert_eq!(from_extension("rs").unwrap().name(), "Rust");
//! assert_eq!(from_extension("md").unwrap().name(), "Markdown");
//!
//! // The case is ignored
//! assert_eq!(from_path("foo.jSoN").unwrap().name(), "JSON");
//! assert_eq!(from_extension("jSoN").unwrap().name(), "JSON");
//! ```
//!
//! # Language ID
//!
//! In short, the language [`id`](struct.Language.html#method.id)
//! is a lowercase version of [`name`](struct.Language.html#method.name).
//! However, it also replaces symbols making it usable as a [URL slug].
//!
//! For instance `foo.hpp` is identified as language name `C++` and
//! language ID `cpp`.
//!
//! [URL slug]: https://en.wikipedia.org/wiki/Clean_URL#Slug
//!
//! ```
//! use detect_lang::from_path;
//! assert_eq!(from_path("foo.rs").unwrap().id(), "rust");
//! assert_eq!(from_path("foo.cpp").unwrap().id(), "cpp");
//! assert_eq!(from_path("foo.hpp").unwrap().id(), "cpp");
//!
//! use detect_lang::from_extension;
//! assert_eq!(from_extension("rs").unwrap().id(), "rust");
//! assert_eq!(from_extension("cpp").unwrap().id(), "cpp");
//! assert_eq!(from_extension("hpp").unwrap().id(), "cpp");
//!
//! // The case is ignored
//! assert_eq!(from_path("foo.jSoN").unwrap().id(), "json");
//! assert_eq!(from_extension("jSoN").unwrap().id(), "json");
//! ```
//!
//! # Always Lowercase
//!
//! If the extension is guaranteed to always be lowercase,
//! then consider using [`from_lowercase_extension`] to avoid
//! allocation and conversion to lowercase.
//!
//! [`from_lowercase_extension`]: fn.from_lowercase_extension.html
//!
//! ```
//! # use detect_lang::{from_extension};
//! use detect_lang::{from_lowercase_extension, Language};
//!
//! assert_eq!(from_lowercase_extension("json"), Some(Language("JSON", "json")));
//! assert_eq!(from_lowercase_extension("jSoN"), None);
//!
//! assert_eq!(from_extension("json"), Some(Language("JSON", "json")));
//! assert_eq!(from_extension("jSoN"), Some(Language("JSON", "json")));
//! ```
//!
//! # Match Example
//!
//! ```
//! use std::path::Path;
//! use detect_lang::{from_path, Language};
//!
//! let path = Path::new("foo.rs");
//! match from_path(path) {
//!     //   Language(name, id)
//!     Some(Language(_, "rust")) => println!("This is Rust"),
//!     Some(Language(..))        => println!("Well it's not Rust"),
//!     None                      => println!("Ehh, what?"),
//! }
//! ```

#![forbid(unsafe_code)]
#![deny(missing_docs)]
// #![deny(missing_doc_code_examples)]
#![deny(missing_debug_implementations)]
#![warn(clippy::all)]

use std::ffi::OsStr;
use std::ops::Deref;
use std::path::Path;

mod languages;

use languages::LANGUAGES;

/// Languages contain a name and an ID (`Language(name, id)`).
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Language<'a>(pub &'a str, pub &'a str);

impl<'a> Language<'a> {
    /// Returns the name of the language.
    ///
    /// # Example
    ///
    /// ```
    /// # use detect_lang::{from_path, from_extension};
    /// assert_eq!(from_path("foo.rs").unwrap().name(), "Rust");
    /// assert_eq!(from_path("foo.md").unwrap().name(), "Markdown");
    ///
    /// assert_eq!(from_extension("rs").unwrap().name(), "Rust");
    /// assert_eq!(from_extension("md").unwrap().name(), "Markdown");
    ///
    /// // The case is ignored
    /// assert_eq!(from_path("foo.jSoN").unwrap().name(), "JSON");
    /// assert_eq!(from_extension("jSoN").unwrap().name(), "JSON");
    /// ```
    #[inline]
    pub fn name(&self) -> &'a str {
        self.0
    }

    /// Returns the ID of the language.
    /// In most cases the language ID is just a lowercase version of the [`name`](#method.name).
    ///
    /// The ID is also usable as a [URL slug].
    ///
    /// [URL slug]: https://en.wikipedia.org/wiki/Clean_URL#Slug
    ///
    /// # Example
    ///
    /// ```
    /// # use detect_lang::{from_path, from_extension};
    /// assert_eq!(from_path("foo.rs").unwrap().id(), "rust");
    /// assert_eq!(from_path("foo.md").unwrap().id(), "markdown");
    ///
    /// assert_eq!(from_extension("rs").unwrap().id(), "rust");
    /// assert_eq!(from_extension("md").unwrap().id(), "markdown");
    ///
    /// // The case is ignored
    /// assert_eq!(from_path("foo.jSoN").unwrap().id(), "json");
    /// assert_eq!(from_extension("jSoN").unwrap().id(), "json");
    /// ```
    #[inline]
    pub fn id(&self) -> &'a str {
        self.1
    }
}

impl<'a> Deref for Language<'a> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.id()
    }
}

/// Identifies a language from a [path extension].
/// The casing of the extension does not affect the result.
/// Returns `None` if the language was not identified.
///
/// Note that `from_path` does not check if the path exists,
/// nor does it attempt to load the file.
///
/// *[See also `from_extension`][from_extension].*
///
/// [from_extension]: fn.from_extension.html
/// [path extension]: https://doc.rust-lang.org/nightly/std/path/struct.Path.html#method.extension
///
/// # Example
///
/// ```
/// # use detect_lang::{from_path, Language};
/// assert_eq!(from_path("foo.rs"), Some(Language("Rust", "rust")));
/// assert_eq!(from_path("foo.md"), Some(Language("Markdown", "markdown")));
/// assert_eq!(from_path("foo.cpp"), Some(Language("C++", "cpp")));
/// assert_eq!(from_path("foo.unknown"), None);
///
/// // The case is ignored
/// assert_eq!(from_path("foo.jSoN"), Some(Language("JSON", "json")));
/// ```
///
/// # Unsupported Language
///
/// If a language is not supported, then feel free to submit an issue
/// on the [issue tracker], or add it to [languages.rs] and submit
/// a [pull request].
///
/// [issue tracker]: https://github.com/vallentin/detect-lang/issues
/// [pull request]: https://github.com/vallentin/detect-lang/pulls
/// [languages.rs]: https://github.com/vallentin/detect-lang/blob/master/src/languages.rs
#[inline]
pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Language<'static>> {
    if let Some(Some(ext)) = path.as_ref().extension().map(OsStr::to_str) {
        from_extension(ext)
    } else {
        None
    }
}

/// Identifies a language from a file extension.
/// The casing of the extension does not affect the result.
/// Returns `None` if the language was not identified.
///
/// *[See also `from_path`][from_path].*
///
/// If the extension is guaranteed to always be lowercase,
/// then consider using [`from_lowercase_extension`] to avoid
/// allocation and conversion to lowercase.
///
/// [from_path]: fn.from_path.html
/// [`from_lowercase_extension`]: fn.from_lowercase_extension.html
///
/// # Example
///
/// ```
/// # use detect_lang::{from_extension, Language};
/// assert_eq!(from_extension("rs"), Some(Language("Rust", "rust")));
/// assert_eq!(from_extension("md"), Some(Language("Markdown", "markdown")));
/// assert_eq!(from_extension("cpp"), Some(Language("C++", "cpp")));
/// assert_eq!(from_extension("unknown"), None);
///
/// // The case is ignored
/// assert_eq!(from_extension("jSoN"), Some(Language("JSON", "json")));
/// ```
///
/// # Unsupported Language
///
/// If a language is not supported, then feel free to submit an issue
/// on the [issue tracker], or add it to [languages.rs] and submit
/// a [pull request].
///
/// [issue tracker]: https://github.com/vallentin/detect-lang/issues
/// [pull request]: https://github.com/vallentin/detect-lang/pulls
/// [languages.rs]: https://github.com/vallentin/detect-lang/blob/master/src/languages.rs
#[inline]
pub fn from_extension<S: AsRef<str>>(extension: S) -> Option<Language<'static>> {
    let ext = extension.as_ref().to_ascii_lowercase();
    from_lowercase_extension(ext)
}

/// Identifies a language from a lowercase file extension.
/// Returns `None` if the language was not identified.
///
/// If the extension is not guaranteed to always be lowercase,
/// then use [`from_extension`] instead.
///
/// *[See also `from_path`][from_path].*
///
/// [from_path]: fn.from_path.html
/// [`from_extension`]: fn.from_extension.html
///
/// # Example
///
/// ```
/// # use detect_lang::{from_extension, from_lowercase_extension, Language};
/// assert_eq!(from_lowercase_extension("rs"), Some(Language("Rust", "rust")));
/// assert_eq!(from_lowercase_extension("md"), Some(Language("Markdown", "markdown")));
/// assert_eq!(from_lowercase_extension("cpp"), Some(Language("C++", "cpp")));
/// assert_eq!(from_lowercase_extension("unknown"), None);
///
/// // Use `from_extension` if casing should be ignored
/// assert_eq!(from_lowercase_extension("jSoN"), None);
/// assert_eq!(from_extension("jSoN"), Some(Language("JSON", "json")));
/// ```
///
/// # Unsupported Language
///
/// If a language is not supported, then feel free to submit an issue
/// on the [issue tracker], or add it to [languages.rs] and submit
/// a [pull request].
///
/// [issue tracker]: https://github.com/vallentin/detect-lang/issues
/// [pull request]: https://github.com/vallentin/detect-lang/pulls
/// [languages.rs]: https://github.com/vallentin/detect-lang/blob/master/src/languages.rs
#[inline]
pub fn from_lowercase_extension<S: AsRef<str>>(extension: S) -> Option<Language<'static>> {
    LANGUAGES
        .binary_search_by_key(&extension.as_ref(), |&(ext, _)| ext)
        .ok()
        .map(|i| LANGUAGES[i].1)
}
