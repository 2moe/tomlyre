use crate::opt::{get_opt_md, get_opt_text, sample_directory};
use clap::Parser;
use getset::Getters;
use std::path::PathBuf;

#[derive(Parser, Debug, Getters)]
#[getset(get = "pub(crate) with_prefix")]
#[command(arg_required_else_help = true)]
#[command(visible_alias = "s")]
#[command(about = get_opt_text("set-opts"))]
#[command(long_about = get_opt_md("set-opts-help"))]
pub(crate) struct SetOpts {
    #[arg(
        hide = true,
        index = 1,
        value_name = sample_directory(false),
        value_hint = clap::ValueHint::FilePath
    )]
    from: PathBuf,

    /// This field can be used to set a key-value pair in a TOML/YAML/JSON file.
    #[arg(
        long,
        num_args = 1..=2,
        default_missing_value = "",
        value_names = ["key","value"],
        groups = ["k", "value"],
        help = get_opt_text("set-kv"),
    )]
    kv: Option<Vec<String>>,

    #[arg(
        short,
        long,
        num_args = 0..=1,
        value_name = "key",
        default_missing_value = "",
        group = "k",
        help = get_opt_text("set-key"),
        long_help = get_opt_text("key-help"),
    )]
    key: Option<String>,

    #[arg(
        long = "ck",
        num_args = 0..=1,
        visible_alias = "concat_key",
        value_name = "Vec<key>",
        // The `default_missing_value` option specifies the default value to use if this argument is not provided.
        default_missing_value = "",
        group = "k",
        help = get_opt_text("concat-key"),
        long_help = get_opt_md("concat-key-help"),
    )]
    concat_key: Option<Vec<String>>,

    /// When the `pre` option is enabled, the output will be previewed after modification.
    #[arg(
        long,
        visible_alias = "preview",
        requires_all = ["k", "value"],
        help = get_opt_text("preview"),
        // long_help = get_text("preview-help"),
    )]
    pre: bool,

    #[arg(
        long,
        visible_alias = "sv",
        help = get_opt_text("set-save"),
    )]
    save: bool,

    /// 保存到指定位置，e.g. `set cfg.json5 -k main --map k1 v1 --save --to cfg.yml`
    #[arg(
        name = "to",
        short = 't',
        long,
        value_name = sample_directory(false),
        value_hint = clap::ValueHint::AnyPath,
        help = get_opt_text("set-save-to"),
        long_help = get_opt_md("set-save-to-help"),
    )]
    save_to: Option<PathBuf>,

    #[arg(
        short,
        long,
        value_name = "string value",
        group = "value",
        help_heading = "Value",
        visible_alias = "string",
        help = get_opt_text("string"),
    )]
    str: Option<String>,

    #[arg(
        short,
        long,
        value_name = "boolean value",
        group = "value",
        help_heading = "Value",
        help = get_opt_md("bool"),
    )]
    bool: Option<bool>,

    #[arg(
        short,
        long,
        value_name = "64-bit integer value",
        group = "value",
        visible_aliases = ["i64", "int"],
        help_heading = "Value",
        allow_negative_numbers = true,
        help = get_opt_text("num"),
    )]
    num: Option<i64>,

    #[arg(
        long,
        value_name = "double float value",
        group = "value",
        help_heading = "Value",
        allow_negative_numbers = true,
        help = get_opt_text("f64"),
    )]
    f64: Option<f64>,

    #[arg(
        short,
        long,
        visible_aliases = ["array", "str-arr"],
        value_name = "Vec<String>: -a str1 -a str2 -a str3...",
        group = "value",
        help_heading = "Value",
        help = get_opt_text("array"),
        long_help = get_opt_md("array-help"),
    )]
    arr: Option<Vec<String>>,

    #[arg(
        long,
        visible_alias = "na",
        value_name = "Vec<i64>",
        group = "value",
        help_heading = "Value",
        allow_negative_numbers = true,
        help = get_opt_text("num-arr"),
        long_help = get_opt_md("num-arr-help"),
    )]
    num_arr: Option<Vec<i64>>,

    #[arg(
        long,
        visible_aliases = ["fa", "double-float-arr"],
        value_name = "Vec<f64>",
        group = "value",
        help_heading = "Value",
        allow_negative_numbers = true,
        help = get_opt_text("f64-arr"),
        long_help = get_opt_md("f64-arr-help"),
    )]
    f64_arr: Option<Vec<f64>>,

    #[arg(
        long,
        visible_alias = "ba",
        value_name = "Vec<bool>",
        group = "value",
        help_heading = "Value",
        help = get_opt_text("bool-arr"),
        long_help = get_opt_md("bool-arr-help"),
    )]
    bool_arr: Option<Vec<bool>>,

    #[arg(
        long,
        visible_alias = "aot",
        // value_name = "usage: --aot 'k1, v1, k2, v2' --aot 'k1, v1'...",
        value_name = r#"Vec<"k1,v1, k2,v2, ..., kn, vn">"#,
        group = "value",
        help_heading = "Value",
        help = get_opt_text("aot"),
        long_help = get_opt_md("aot-help"),
    )]
    array_of_tables: Option<Vec<String>>,

    /// usage: --saot k1 v1 --saot k2 v2...
    #[arg(
        long,
        num_args = 2,
        visible_alias = "saot",
        value_names = ["k", "v"],
        group = "value",
        help_heading = "Value",
        help = get_opt_text("saot"),
        long_help = get_opt_md("saot-help"),
    )]
    single_array_of_tables: Option<Vec<String>>,

    /// Vec<String-Key, String-Value>
    #[arg(
        short = 'm',
        long,
        num_args = 2,
        visible_aliases = ["map", "tb"],
        value_names = ["key", "value"], 
        group = "value",
        help_heading = "Value",
        help = get_opt_text("table"),
        long_help = get_opt_md("table-help"),
    )]
    table: Option<Vec<String>>,

    /// Vec<String-Key, String-Value>
    #[arg(
        short,
        long,
        num_args = 2,
        visible_aliases = ["imap", "itb"],
        value_names = ["key", "value"],
        group = "value",
        help_heading = "Value",
        help = get_opt_text("inline-table"),
        long_help = get_opt_md("inline-table-help"),
    )]
    inline_table: Option<Vec<String>>,

    /// e.g. "--datetime '2022-12-21 12:21:02Z'"
    #[arg(
        // 
        long,
        value_name = "rfc3339",
        group = "value",
        help_heading = "Value",
        help = get_opt_text("datetime"),
        long_help = get_opt_md("datetime-help"),
    )]
    datetime: Option<String>,

    #[arg(
        long,
        group = "value",
        help_heading = "Value",
        visible_aliases = ["rm", "remove"],
        help = get_opt_text("none"),
        long_help = get_opt_md("none-help"),
    )]
    none: bool,
}
