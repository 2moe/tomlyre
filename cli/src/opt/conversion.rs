use crate::{
    assets::get_l10n_text,
    opt::{get_md, sample_directory},
};
use clap::Parser;
use getset::Getters;
use glossa::GetText;
use std::path::PathBuf;

#[derive(Parser, Debug, Getters)]
#[getset(get = "pub(crate) with_prefix")]
#[command(arg_required_else_help = true)]
#[command(visible_aliases = ["c", "convert"])]
#[command(about = get_conv_text("conv-opts"))]
pub(crate) struct ConvOpts {
    #[arg(
        // The `index` option specifies the position of this argument in the command-line arguments.
        // In this case, it is the second argument (after the program name).
        index = 1,
        // The `num_args` option specifies how many values this argument should consume.
        // In this case, it consumes a single value (the input file path).
        num_args = 1,
        // The `value_name` option specifies the name of the value that this argument consumes.
        // In this case, it is a directory that contains sample files (which is used for formatting help messages).
        value_name = sample_directory(false),
        value_hint = clap::ValueHint::FilePath,
        help = get_conv_text("from"),
    )]
    from: PathBuf,

    /// This field can be used to specify multiple output formats/files at once.
    /// By default, no files are saved.
    #[arg(
        short,
        long,
        value_name = "-t toml -t yml -t a.json",
        value_hint = clap::ValueHint::AnyPath,
        help = get_conv_text("conv-to"),
        long_help = get_conv_md("conv-to-help"),
    )]
    to: Option<Vec<PathBuf>>,

    /// If this option is enabled, the output will be saved to the files specified by the `to` field.
    #[arg(
        // 
        short,
        long,
        visible_alias = "sv",
        help = get_conv_text("save"),
    )]
    save: bool,
}

fn get_conv_text<'k: 'res, 'res>(key: &'k str) -> Option<&'res str> {
    get_l10n_text()
        .get("conv", key)
        .ok()
}

fn get_conv_md(key: &str) -> String {
    let md_map = "conv_md";
    let text_map = "conv";

    get_md(md_map, key, text_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_conv_help() {
        let s = get_conv_md("conv-to-help");
        println!("{s}");
    }
}
