use std::path::PathBuf;

use crate::opt::{get_opt_md, get_opt_text, sample_directory};
use clap::Parser;
use getset::Getters;

/// `tomlyre get /path/to/file -k x.y.z`
#[derive(Parser, Debug, Getters)]
#[getset(get = "pub(crate) with_prefix")]
#[command(arg_required_else_help = true)]
#[command(visible_aliases = ["g", "query"])]
#[command(about = get_opt_text("get-opts"))]
#[command(long_about = get_opt_text("get-opts-help"))]
pub(crate) struct GetOpts {
    #[arg(
        // The `hide` option hides this argument from help messages.
        hide = true,
        index = 1,
        value_name = sample_directory(false),
        value_hint = clap::ValueHint::FilePath,
    )]
    from: PathBuf,

    #[arg(
        short,
        long,
        num_args = 0..=1,
        value_name = "key",
        default_missing_value = "",
        group = "k",
        help = get_opt_text("key"),
        long_help = get_opt_text("key-help"),
    )]
    key: Option<String>,

    #[arg(
        long = "ck",
        num_args = 0..=1,
        visible_alias = "concat_key",
        value_name = "Vec<key>",
        default_missing_value = "",
        group = "k",
        help = get_opt_text("concat-key"),
        long_help = get_opt_md("concat-key-help"),
    )]
    concat_key: Option<Vec<String>>,
}
