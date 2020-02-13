use crate::Language;

macro_rules! l {
    ($ext:literal, $name:literal, $id:literal) => {
        ($ext, Language($name, $id))
    };
}

// The array is sorted by the extension name
pub(crate) const LANGUAGES: [(&str, Language); 58] = [
    l!("bat", "Batch", "batch"),
    l!("c", "C", "c"),
    l!("cc", "C++", "cpp"),
    l!("cl", "Common Lisp", "common-lisp"),
    l!("clj", "Clojure", "clojure"),
    l!("comp", "GLSL", "glsl"),
    l!("cpp", "C++", "cpp"),
    l!("cs", "C#", "csharp"),
    l!("css", "CSS", "css"),
    l!("cxx", "C++", "cpp"),
    l!("dart", "Dart", "dart"),
    l!("frag", "GLSL", "glsl"),
    l!("geom", "GLSL", "glsl"),
    l!("glsl", "GLSL", "glsl"),
    l!("go", "Go", "go"),
    l!("h", "C", "c"),
    l!("haml", "Haml", "haml"),
    l!("handlebars", "Handlebars", "handlebars"),
    l!("hbs", "Handlebars", "handlebars"),
    l!("hlsl", "HLSL", "HLSL"),
    l!("hpp", "C++", "cpp"),
    l!("html", "HTML", "html"),
    l!("hxx", "C++", "cpp"),
    l!("ini", "INI", "ini"),
    l!("java", "Java", "java"),
    l!("jinja", "Jinja", "jinja"),
    l!("jinja2", "Jinja", "jinja"),
    l!("js", "JavaScript", "javascript"),
    l!("json", "JSON", "json"),
    l!("jsonc", "JSON with Comments", "jsonc"),
    l!("kt", "Kotlin", "kotlin"),
    l!("less", "Less", "less"),
    l!("lua", "Lua", "lua"),
    l!("md", "Markdown", "markdown"),
    l!("pl", "Perl", "perl"),
    l!("py", "Python", "python"),
    l!("pyc", "Python", "python"),
    l!("pyo", "Python", "python"),
    l!("rb", "Ruby", "ruby"),
    l!("rkt", "Racket", "racket"),
    l!("rs", "Rust", "rust"),
    l!("sass", "SASS", "sass"),
    l!("sc", "Scala", "scala"),
    l!("scala", "Scala", "scala"),
    l!("scss", "SCSS", "scss"),
    l!("sh", "Shell", "shell"),
    l!("sql", "SQL", "sql"),
    l!("swift", "Swift", "swift"),
    l!("tesc", "GLSL", "glsl"),
    l!("tese", "GLSL", "glsl"),
    l!("tex", "TeX", "tex"),
    l!("toml", "TOML", "toml"),
    l!("ts", "TypeScript", "typescript"),
    l!("vert", "GLSL", "glsl"),
    l!("xhtml", "XHTML", "xhtml"),
    l!("xml", "XML", "xml"),
    l!("yaml", "YAML", "yaml"),
    l!("yml", "YAML", "yaml"),
];

#[test]
fn check_order() {
    for (a, b) in LANGUAGES.iter().zip(LANGUAGES.iter().skip(1)) {
        assert!(
            a.0 < b.0,
            "Languages out of order - {:?} should come after {:?}",
            a,
            b,
        );
    }
}
