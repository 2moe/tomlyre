use crate::{
    assets::get_l10n_text,
    opt::{conversion::ConvOpts, get::GetOpts, get_md, set::SetOpts, PKG_VERSION},
};
use clap::{ColorChoice, Parser, Subcommand};
use getset::Getters;
use glossa::GetText;
use std::path::PathBuf;

#[derive(Parser, Debug, Getters)]
#[getset(get = "pub(crate) with_prefix")]
#[command(arg_required_else_help = true)]
#[command(color = ColorChoice::Always)]
pub(crate) struct Cli {
    /// Specifies the name of the theme to use.
    #[arg(
        short,
        long,
        value_name = r#""Monokai Extended", "ayu-dark""#,
        group = "theme-name",
        num_args = 0..=1,
        default_missing_value = "",
        help = get_args_text("theme"),
        long_help = get_args_md("theme-help"),
    )]
    theme: Option<String>,

    /// Specifies a custom theme file to use.
    ///
    /// Must be used in conjunction with the `--theme` option to specify the theme name.
    #[arg(
        long,
        value_name = "/path/to/theme_file.dump",
        value_hint = clap::ValueHint::FilePath,
        group = "theme-file",
        visible_alias = "tf",
        requires = "theme",
        help = get_args_text("theme-file"),
        long_help = get_args_md("theme-file-help"),
    )]
    theme_file: Option<PathBuf>,

    /// Specifies a custom uncompressed theme file to use.
    ///
    /// Much like `theme_file`, but the file pointed to is uncompressed data.
    #[arg(
        long,
        value_name = "/path/to/theme_file.dump",
        value_hint = clap::ValueHint::FilePath,
        group = "theme-file",
        visible_alias = "tuf",
        help = get_args_text("theme-uncompressed-file"),
        long_help = get_args_md("theme-uncompressed-file-help"),
    )]
    theme_uncompressed_file: Option<PathBuf>,

    #[arg(
        long,
        visible_alias = "dis-theme-bg",
        help = get_args_text("disable-theme-background"),
        long_help = get_args_md("disable-theme-background-help"),
    )]
    disable_theme_background: bool,

    /// Specifies the style of tables to use.
    #[arg(
    long,
    value_name = "u8, md",
    num_args = 0..=1,
    default_missing_value = "",
    visible_alias = "ts",
    help = get_args_text("table-style"),
    long_help = get_args_md("table-style-help"),
    )]
    table_style: Option<String>,

    /// Specify the format manually.
    ///
    /// When reading from standard input (e.g. `cat a | tomlyre conv -`), the format of the input data may not be known.
    /// In such cases, this field can be used to manually specify the format.
    #[arg(
        long,
        // It can be one of several format names (e.g. 'toml', 'yaml', 'json').
        value_name = "toml/yaml/json/..",
        visible_alias = "src-fmt",
        help = get_args_text("from-format"),
        long_help = get_args_md("from-format-help"),
    )]
    from_format: Option<String>,

    /// Generates shell completion scripts for specified shells. Accepts the names of supported shells as its value. Has an additional visible alias of `sh-comp`.
    #[arg(
        long,
        value_name = "zsh, fish, pwsh, bash, elvish",
        visible_alias = "sh-comp",
        help = get_args_text("shell-completion"),
        // long_help = get_text("shell-completion-help"),
        long_help = get_args_md("shell-completion-help"),
    )]
    shell_completion: Option<String>,

    /// Prints the version number of the tool. Can be specified with either a short or long option flag.
    #[arg(long, short = 'V', help = PKG_VERSION)]
    version: bool,

    /// Specifies which subcommand to run. Can be any one of the valid subcommands, represented by their respective enums.
    #[command(subcommand)]
    sub: Option<Sub>,
}

#[derive(Debug, Subcommand)]
#[command(arg_required_else_help = true)]
pub(crate) enum Sub {
    Conv {
        #[command(flatten)]
        opt: ConvOpts,
    },
    Get {
        #[command(flatten)]
        opt: GetOpts,
    },
    Set {
        #[command(flatten)]
        // This is a Large variant, So put it on the heap.
        opt: Box<SetOpts>,
    },
}

fn get_args_text<'k: 'res, 'res>(key: &'k str) -> Option<&'res str> {
    get_l10n_text()
        .get("args", key)
        .ok()
}

fn get_args_md(key: &str) -> String {
    let md_map = "args_md";
    let text_map = "args";

    get_md(md_map, key, text_map)
}
