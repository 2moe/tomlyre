use crate::opt::{args::Cli, set::SetOpts};
use anyhow::Result;
use hlight::HighLightRes;
use std::{borrow::Cow, process::exit, str::FromStr};
use tomlyre::set::{
    Array, ArrayOfTables, CfgOpts, Datetime, InlineTable, TomlKey, TomlTable,
    TomlValue, Value,
};

/// The function first calls the `set_value` method on the provided `cfg_opts` with the input value converted into a `Some` option using the `Into` trait.
/// If this succeeds, indicating that the value was successfully set in the `CfgOpts`, the function then calls the `exit` function with a status code of 0, terminating the program.
fn set_value_and_exit<V: Into<Value>>(cfg_opts: &CfgOpts, v: V) -> Result<()> {
    cfg_opts.set_value(Some(v))?;
    exit(0);
}

fn to_toml_array<T: Copy>(v: &[T]) -> Array
where
    tomlyre::set::Value: std::convert::From<T>,
{
    Array::from_iter(v.iter().copied())
}

/// Handles "set" options by setting values in the `CfgOpts` struct.
///
/// The function first determines the key to use for the configuration value, based on whether the `key` or `concat_key` option was provided.
/// If the `key` option was provided, it is used as a string key.
/// If the `concat_key` option was provided, it is split into components and used as a vector key.
/// If neither option was provided, an empty string key is used. The resulting `TomlKey` is then included in the new `CfgOpts` instance along with other arguments extracted from the input `opt`.
///
/// Then checks which type of value to set based on the provided options. If the `pre` flag is provided, the preview mode is enabled by calling `cfg.get_preview_mut()` and setting it to `true`.
///
/// If the `kv` option is provided, the corresponding key-value pair is extracted from the input `opt` and passed to the `set_value_and_exit` function along with the `CfgOpts` instance.
///
/// If the `str`, `num`, `f64`, `bool`, `datetime`, `arr`, `num_arr`, `f64_arr`, `bool_arr`, `inline_table`, `table`, `array_of_tables`, `none`, or `single_array_of_tables` option is provided, the corresponding value is extracted from the input `opt` and converted to the appropriate data type using the relevant conversion function.
/// Then set the value, and exit.
///
/// If no value is set, the function sets the `set_value` flag to `false` and calls `cfg.set_value::<bool>(None)?` to update the `CfgOpts` instance.
/// This prints the entire configuration file.
///
/// Finally, the function returns `Ok(())`.
pub(crate) fn handle_setopts(
    opt: &SetOpts,
    hl: &HighLightRes,
    table_style: Cow<str>,
    args: &Cli,
) -> Result<()> {
    let key = match (opt.get_key().as_ref(), opt.get_concat_key().as_ref()) {
        (Some(s), _) => TomlKey::Str(Cow::from(s)),
        (_, Some(c)) => TomlKey::Vec(c),
        _ => TomlKey::Str(Cow::from("")),
    };

    let mut cfg = CfgOpts::new(
        opt.get_from(),
        args.get_from_format().as_ref(),
        *opt.get_save(),
        opt.get_save_to().as_ref(),
        key,
        Some(hl),
        table_style.as_ref(),
    );

    if *opt.get_pre() {
        *cfg.get_preview_mut() = true;
    }

    if let Some(kv_pair) = opt.get_kv() {
        *cfg.get_key_mut() = TomlKey::Str(Cow::from(&kv_pair[0]));
        cfg.set_value(
            kv_pair
                .get(1)
                .map_or(Some(""), |v| Some(v)),
        )?;
        exit(0)
    }

    if let Some(s) = opt.get_str() {
        set_value_and_exit(&cfg, s)?;
    }

    if let Some(s) = opt.get_num() {
        set_value_and_exit(&cfg, *s)?;
    }

    if let Some(s) = opt.get_f64() {
        set_value_and_exit(&cfg, *s)?;
    }

    if let Some(s) = opt.get_bool() {
        set_value_and_exit(&cfg, *s)?;
    }

    if let Some(s) = opt.get_datetime() {
        set_value_and_exit(&cfg, Datetime::from_str(s)?)?;
    }

    if let Some(s) = opt.get_arr() {
        set_value_and_exit(&cfg, Array::from_iter(s))?;
    }

    if let Some(s) = opt.get_num_arr() {
        set_value_and_exit(&cfg, to_toml_array(s))?;
    }

    if let Some(s) = opt.get_f64_arr() {
        set_value_and_exit(&cfg, to_toml_array(s))?;
    }

    if let Some(s) = opt.get_bool_arr() {
        set_value_and_exit(&cfg, to_toml_array(s))?;
    }

    if let Some(s) = opt.get_inline_table() {
        set_value_and_exit(&cfg, InlineTable::from_iter(get_map(s)))?;
    }

    // The following type cannot be used with `set_value_and_exit()`
    if let Some(s) = opt.get_table() {
        *cfg.get_value_mut() = TomlValue::Table(TomlTable::from_iter(get_map(s)));
        cfg.set_value::<bool>(None)?;
        exit(0)
    }

    if let Some(s) = opt.get_array_of_tables() {
        let iter = s
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| {
                let v = x
                    .split(',')
                    .map(|x| x.trim().to_owned())
                    .collect::<Vec<_>>();
                TomlTable::from_iter(get_map(&v))
            });

        *cfg.get_value_mut() = TomlValue::AOT(ArrayOfTables::from_iter(iter));
        cfg.set_value::<bool>(None)?;
        exit(0)
    }

    if *opt.get_none() {
        *cfg.get_value_mut() = TomlValue::NONE;
        cfg.set_value::<bool>(None)?;
        exit(0)
    }

    if let Some(s) = opt.get_single_array_of_tables() {
        let iter = s
            .chunks_exact(2)
            .map(|c| (c[0].to_owned(), c[1].to_owned()))
            .map(|arr| TomlTable::from_iter([arr]));

        *cfg.get_value_mut() = TomlValue::AOT(ArrayOfTables::from_iter(iter));

        cfg.set_value::<bool>(None)?;
        exit(0)
    }

    {
        *cfg.get_set_value_mut() = false;
        cfg.set_value::<bool>(None)?;
    }

    Ok(())
}

fn get_map(s: &[String]) -> Vec<(String, String)> {
    s.chunks_exact(2)
        .map(|c| (c[0].to_owned(), c[1].to_owned()))
        .collect()
}
