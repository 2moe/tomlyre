use crate::{assets::get_l10n_text, opt::args::Cli};
use glossa::{assets::OnceCell, GetText};
use hlight::{
    gen_syntax_highlight,
    theme::{dumps, ThemeSet},
    HighLightRes,
};
use owo_colors::OwoColorize;
use std::{
    borrow::Cow,
    env,
    io::{BufWriter, Write},
    path::PathBuf,
    process::exit,
};

const SAMPLE_HIGHLIGHT_TEXT: &str = r#"
[[bin]]
doc = false
path = "src/main.rs"

[features]
default = ["conversion"]
conversion = []

[dependencies]
tomlyre = { path = "../core" }

[package.metadata.test]
num = 233
float = 314e-2
float2 = 3.14
hex = 0xFF0000
time = 2023-04-11 21:12:44Z
"😎" = """
⚗️
✅
"""
"#;

/// Retrieves the names of all themes in a given `ThemeSet` object and returns them as a `Vec<String>`.
fn get_theme_list(set: &ThemeSet) -> Vec<String> {
    set.themes
        .keys()
        .cloned()
        .collect()
}

impl Cli {
    /// Retrieves a `ThemeSet` object that contains the definitions of syntax highlighting themes.
    fn get_theme_set(&self) -> &'static ThemeSet {
        let uncompressed = self
            .get_theme_uncompressed_file()
            .as_ref();
        let compressed = self.get_theme_file().as_ref();

        match get_theme_from_dump_file(uncompressed, compressed)
            .or_else(get_theme_from_os_env)
        {
            Some(s) => custom_static_set(s),
            _ => HighLightRes::static_theme_set(),
        }
    }

    /// Parses the style of a syntax highlighting theme and returns a `HighLightRes` object that contains the parsed information.
    ///
    /// The function begins by calling the `get_theme_set` method to retrieve the `ThemeSet` object that contains the syntax highlighting theme definitions.
    ///
    /// It then uses the `match` to check if a theme name has been provided.
    /// If no theme name is given, the function will print out a list of available themes and exit the program.
    ///
    /// If a theme name is provided, the function checks if the theme exists in the `ThemeSet`.
    /// If it does, a new `HighLightRes` object containing the theme's information is returned.
    /// If the theme does not exist in the `ThemeSet`, an error message is logged, the available themes are listed, and a default theme is selected and returned.
    ///
    /// The function also includes code for printing out syntax highlighted text using the `print_syntax_highlight` function. This code is only executed if no theme name is given and a list of available themes is printed.
    pub(crate) fn parse_theme_style(&self) -> HighLightRes {
        let theme_set = self.get_theme_set();

        match self.get_theme() {
            Some(s) if s.trim().is_empty() => {
                let mut res = HighLightRes::new(Cow::from(""), theme_set);
                let name_list = get_theme_list(res.get_theme_set());

                let mut out = BufWriter::new(std::io::stdout().lock());
                const MSG: &str = "Failed to write/flush stdio output stream";

                for name in &name_list {
                    // out.write_all(b"\n").expect(MSG);
                    log::trace!("{}", name);
                    writeln!(out, "\ntheme: {}", name.bright_yellow()).expect(MSG);

                    *res.get_name_mut() = Cow::from(name);
                    *res.get_theme_mut() = OnceCell::new();

                    gen_syntax_highlight(
                        "toml",
                        SAMPLE_HIGHLIGHT_TEXT,
                        Some(&res),
                        Some(&mut out),
                    )
                    .expect("Failed to set highlight");
                }
                out.flush().expect(MSG);
                exit(0)
            }
            Some(s) => check_theme(theme_set, s),
            _ => get_highlight_resource_from_os_env().0,
        }
    }
}

/// The function first uses the `match` to check if a dump file and an uncompressed dump file exist for the themes. If a compressed dump file exists and there is no uncompressed dump file, the function will use the `from_dump_file` function from the `dumps` module to deserialize the contents of the dump file and retrieve the `ThemeSet` object.
///
/// If an uncompressed dump file exists, the function will use the `from_uncompressed_dump_file` function from the `dumps` module to retrieve the `ThemeSet` object.
///
/// If neither a compressed nor uncompressed dump file exists, the function will return the default `ThemeSet` object provided by the `highlight` module.
fn get_theme_from_dump_file(
    uncompressed: Option<&PathBuf>,
    compressed: Option<&PathBuf>,
) -> Option<ThemeSet> {
    match (uncompressed, compressed) {
        (Some(u), None) if u.exists() => dumps::from_uncompressed_dump_file(u)
            // .expect("Failed to get uncompressed dump file")
            .ok(),
        (_, Some(b)) if b.exists() => {
            dumps::from_dump_file(b).ok()
            // .expect("Failed to get dump file")
        }
        _ => None,
    }
}

pub(crate) fn get_static_theme_style<'a>() -> &'static HighLightRes<'a> {
    static V: OnceCell<HighLightRes> = OnceCell::new();
    V.get_or_init(|| {
        let (theme, _custom) = get_highlight_resource_from_os_env();
        // if !custom {
        //     *theme.get_name_mut() = Cow::from(HighLightRes::ayu_dark_theme_name());
        //     *theme.get_background_mut() = false;
        // }

        theme
    })
}

fn get_theme_from_os_env() -> Option<ThemeSet> {
    get_theme_from_dump_file(
        get_os_env_theme_uncompressed_file().as_ref(),
        get_os_env_theme_file().as_ref(),
    )
}

pub(crate) fn get_highlight_resource_from_os_env<'a>() -> (HighLightRes<'a>, bool) {
    let (mut res, is_custom) = match (get_env_theme_name(), get_theme_from_os_env())
    {
        (Some(s), Some(set)) => (check_theme(custom_static_set(set), s), true),
        (Some(s), _) => (check_theme(HighLightRes::static_theme_set(), s), true),
        _ => (HighLightRes::default(), false),
    };

    if !get_env_theme_bg() {
        *res.get_background_mut() = false;
    }

    (res, is_custom)
}

fn custom_static_set(s: ThemeSet) -> &'static ThemeSet {
    static S: OnceCell<ThemeSet> = OnceCell::new();
    S.get_or_init(|| s)
}

fn check_theme<'a>(theme_set: &'a ThemeSet, name: &'a str) -> HighLightRes<'a> {
    if theme_set
        .themes
        .contains_key(name)
        || name == "None"
    {
        HighLightRes::new(Cow::from(name), theme_set)
    } else {
        log::error!(
            "{}: {}",
            get_l10n_text().get_or_default("theme", "invalid-name"),
            name
        );
        log::warn!("theme list: {:?}", get_theme_list(theme_set));

        let s = theme_set
            .themes
            .keys()
            .next()
            .expect("Invalid theme set");

        HighLightRes::new(Cow::from(s), theme_set)
    }
}

fn get_env_theme_name() -> &'static Option<String> {
    static V: OnceCell<Option<String>> = OnceCell::new();
    V.get_or_init(|| env::var("TOMLYRE_THEME").ok())
}

pub(crate) fn get_env_theme_bg() -> bool {
    static V: OnceCell<bool> = OnceCell::new();
    *V.get_or_init(|| {
        env::var("TOMLYRE_THEME_BG")
            .ok()
            .and_then(|x| x.parse::<bool>().ok())
            .unwrap_or(true)
    })
}

fn get_os_env_theme_file() -> &'static Option<PathBuf> {
    static V: OnceCell<Option<PathBuf>> = OnceCell::new();
    V.get_or_init(|| env::var_os("TOMLYRE_THEME_FILE").map(PathBuf::from))
}

fn get_os_env_theme_uncompressed_file() -> &'static Option<PathBuf> {
    static V: OnceCell<Option<PathBuf>> = OnceCell::new();
    V.get_or_init(|| {
        env::var_os("TOMLYRE_THEME_UNCOMPRESSED_FILE").map(PathBuf::from)
    })
}
