use crate::{
    assets::get_l10n_text,
    opt::theme::{
        get_env_theme_bg, get_highlight_resource_from_os_env, get_static_theme_style,
    },
};
use glossa::GetText;
use std::env;
use tomlyre::highlight::{output::get_syntax_highlight, HighLightRes};

pub(crate) const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) mod args;
pub(crate) mod conversion;
pub(crate) mod get;
pub(crate) mod set;
pub(crate) mod theme;

fn get_opt_text<'k: 'res, 'res>(key: &'k str) -> Option<&'res str> {
    get_l10n_text()
        .get("opt", key)
        .ok()
}

fn get_opt_md(key: &str) -> String {
    let md_map = "opt_md";
    let text_map = "opt";

    get_md(md_map, key, text_map)
}

fn get_md(md_map: &str, key: &str, text_map: &str) -> String {
    let highlight_text = || {
        let text = get_l10n_text().get_or_default(md_map, key);
        text.into_owned()
    };

    // non-custom theme && enabled background:
    if !get_highlight_resource_from_os_env().1 && get_env_theme_bg() {
        return highlight_text();
    }

    let style = get_static_theme_style();
    if style.get_name().as_ref() == HighLightRes::monokai_theme_name()
        && get_env_theme_bg()
    {
        return highlight_text();
    }

    let text = get_l10n_text().get_or_default(text_map, key);

    let mut v = Vec::with_capacity(text.len());

    get_syntax_highlight("md", text.as_ref(), Some(style), Some(&mut v))
        .unwrap_or_else(|e| log::warn!("Failed to get `{}`\nErr: {e}", key));

    String::from_utf8(v).unwrap_or_else(|e| {
        log::warn!("{e}");
        "".to_owned()
    })
}

/// It is used to specify the sample values associated with the path.
///
/// The function uses the `#[cfg]` attribute to conditionally compile different code depending on the operating system. If the OS is not Windows, it returns a Unix-style path, and if the OS is Windows, it returns a Windows-style path.
const fn sample_directory(b: bool) -> &'static str {
    #[cfg(not(windows))]
    match b {
        true => "/path/to/directory",
        _ => "/path/to/file",
    }

    #[cfg(windows)]
    match b {
        true => r"C:\path\to\directory",
        _ => r"C:\path\to\file",
    }
}
