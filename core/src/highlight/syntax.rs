use glossa::assets::OnceCell;
use syntect::parsing::{SyntaxReference, SyntaxSet};
pub use syntect::{dumps, highlighting::ThemeSet};

type OnceSyntax = OnceCell<&'static SyntaxReference>;

const SUBLIME_SYNTAXES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/syntect/syntax-set.packdump"
));

/// Finds and returns the appropriate syntax highlighting definition from a `SyntaxSet` based on a given destination format. If not found, it will fallback to json.
///
/// The function first tries to find the syntax highlighting definition based on the file extension of the destination format using the `find_syntax_by_extension` method of the `SyntaxSet`.
pub(crate) fn find_syntax<'a>(
    syntax_set: &'a SyntaxSet,
    dst_fmt: &str,
) -> &'a SyntaxReference {
    syntax_set
        .find_syntax_by_extension(dst_fmt)
        .unwrap_or_else(|| {
            let to_json = || syntax_set.find_syntax_by_extension("json");
            match dst_fmt {
                "sexp" | "lexpr" => syntax_set
                    .find_syntax_by_extension("lisp")
                    .or_else(to_json),
                _ => to_json(),
            }
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text())
        })
}

pub(crate) fn match_static_syntax(
    set: &'static SyntaxSet,
    fmt: &str,
) -> &'static SyntaxReference {
    match fmt {
        "md" | "markdown" => get_markdown(set),
        "toml" => get_toml(set),
        "yaml" | "yml" => get_yaml(set),
        "json" | "ron" => get_json(set),
        _ => find_syntax(set, fmt),
    }
}

fn find_syntax_name<'a>(set: &'a SyntaxSet, name: &str) -> &'a SyntaxReference {
    set.find_syntax_by_name(name)
        .unwrap_or_else(|| set.find_syntax_plain_text())
}

fn get_markdown(set: &'static SyntaxSet) -> &'static SyntaxReference {
    static S: OnceSyntax = OnceCell::new();
    S.get_or_init(|| find_syntax_name(set, "Markdown"))
}

fn get_json(set: &'static SyntaxSet) -> &'static SyntaxReference {
    static S: OnceSyntax = OnceCell::new();
    S.get_or_init(|| find_syntax_name(set, "JSON"))
}

fn get_yaml(set: &'static SyntaxSet) -> &'static SyntaxReference {
    static S: OnceSyntax = OnceCell::new();
    S.get_or_init(|| find_syntax_name(set, "YAML"))
}
fn get_toml(set: &'static SyntaxSet) -> &'static SyntaxReference {
    static S: OnceSyntax = OnceCell::new();
    S.get_or_init(|| find_syntax_name(set, "TOML"))
}

pub(crate) fn static_syntax_set() -> &'static SyntaxSet {
    static S: OnceCell<SyntaxSet> = OnceCell::new();
    S.get_or_init(default_syntax_set)
}

pub fn default_syntax_set() -> SyntaxSet {
    dumps::from_uncompressed_data(SUBLIME_SYNTAXES).unwrap()
}
