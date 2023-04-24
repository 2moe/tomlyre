use crate::{
    opt::{
        args::{Cli, Sub},
        conversion::ConvOpts,
        get::GetOpts,
    },
    parser::set_opts::handle_setopts,
};
use anyhow::Result;
use std::borrow::Cow;
use tomlyre::{
    conversion, get::get_config_file, highlight::HighLightRes, set::TomlKey,
};

/// All subcommands are handled here.
///
/// The main logic is handled in subfunctions, e.g. `Conv` is handled in `handle_conversion_opts()`.
pub(crate) fn handle_sub_cmds(
    opt_sub: Option<&Sub>,
    hl: &HighLightRes,
    table_style: Cow<str>,
    args: &Cli,
) -> Result<()> {
    let Some(cmd) = opt_sub else { return Ok(()) };

    match cmd {
        Sub::Conv { opt } => handle_conversion_opts(opt, hl, args),
        Sub::Get { opt } => handle_getopts(opt, hl, table_style, args),
        Sub::Set { opt } => handle_setopts(opt, hl, table_style, args),
    }
}

/// Handles "get" options by calling the `get_config_file()`.
///
/// The `get_config_file()` is responsible for retrieving configuration values from a file based on the provided arguments. If the function succeeds, returning `Ok(_)`, the closure passed to the `map` method simply maps it to `Ok(())`, which is then returned.
fn handle_getopts(
    opt: &GetOpts,
    hl: &HighLightRes,
    style: Cow<str>,
    args: &Cli,
) -> Result<()> {
    let key = match (opt.get_key(), opt.get_concat_key()) {
        (None, None) => Cow::from(""),
        (Some(k), _) => Cow::from(k),
        (_, Some(ck)) => Cow::from(
            TomlKey::Vec(ck)
                .concat_key()
                .into_owned(),
        ),
    };

    get_config_file(
        &key,
        opt.get_from(),
        args.get_from_format().as_ref(),
        Some(hl),
        style.as_ref(),
    )
    .map(|_| ())
}

/// Handles conversion options by calling the `task` function from the `conversion` module.
///
/// The function calls the `task` function with several args extracted from the input `opt`:
///
/// - `from`: Source of the configuration file
/// - `to`: Format or file path of output
/// - `hl`: Syntax HighLight Resource
///
/// The `task` function from the `conversion` module is responsible for performing the actual conversion based on the provided arguments.
fn handle_conversion_opts(
    opt: &ConvOpts,
    hl: &HighLightRes,
    args: &Cli,
) -> Result<()> {
    conversion::task(
        opt.get_from(),
        opt.get_to().as_ref(),
        *opt.get_save(),
        args.get_from_format().as_ref(),
        Some(hl),
    )
}
