use crate::{
    assets::get_l10n_text,
    conversion::{
        convert::convert_data_format,
        fmt::{auto_detect_unknown_fmt, get_src_format},
        get_static_stdin_data, is_src_stdin,
        serialisation::deser_yaml,
        ConvFmt,
    },
    // highlight::HighLightRes,
    set::get_set_text,
    table::set_header,
};

use anyhow::Result;
use comfy_table::Table;
use getset::Getters;
use glossa::GetText;
use hlight::HighLightRes;
use log::info;
use owo_colors::OwoColorize;
use std::{fs, path::Path};
// use toml::Value;
// use ron::Value;
use serde_yaml::Value;

#[derive(Getters, Debug)]
#[getset(get = "pub with_prefix")]
pub struct ConvertedContents<'f> {
    pub(crate) contents: String,
    pub(crate) fmt: &'f str,
}

impl<'f> ConvertedContents<'f> {
    fn new(contents: String, fmt: &'f str) -> Self {
        Self { contents, fmt }
    }
}

/// Generates a table from src and the associated key.  
///
/// It first converts the src file to a toml string.  
/// If the key is empty, then it will only print the full toml document (with syntax highlighting).
/// If the key is "." , then it will output a complete table.
/// If key is a valid string, then it will find the data in the table by key.
///
/// It takes in several arguments such as a key, source path, source format, highlight resource, and table style.
///
/// It determines the format of the input data and converts it to TOML if necessary using various utility functions.
///
/// Then uses a closure to traverse the TOML data and add its key-value pairs to a table.
///
/// If any key-value pairs are added to the table, it prints the table.
///
/// If the `key` argument is empty, it only highlights and shows the input data without traversing it or converting it.
///
/// Finally, the function returns a `Result` object containing a `ConvertedContents` object which contains the converted TOML data and its original format.
pub fn get_config_file<'a>(
    key: &str,
    src: &'a Path,
    src_format: Option<&'a String>,
    high_light_resource: Option<&HighLightRes>,
    table_style: &str,
) -> Result<ConvertedContents<'a>> {
    let mut cfg_str = String::with_capacity(256);

    // Check if the input source is from stdin.
    let is_stdin = is_src_stdin(src);

    let src_fmt = match src_format {
        // If an optional `src_format` is provided and it's not "bson" while the source is stdin, it saves the `src_format` to `cfg_str`.
        Some(s) if is_stdin && s != "bson" => {
            cfg_str = get_static_stdin_data().to_owned();
            s
        }
        Some(s) => {
            info!("src-fmt: {}", s);
            s
        }
        // If no `src_format` is provided and the source is stdin, it uses `auto_detect_unknown_fmt` to determine the format of the input data.
        _ if is_stdin => {
            let data = get_static_stdin_data();
            cfg_str = data.to_owned();
            auto_detect_unknown_fmt(data)
        }
        // If the input is not from stdin, it uses the `get_src_format` function to determine the format of the input.
        _ => match get_src_format(src) {
            x if x.is_empty() => {
                cfg_str = fs::read_to_string(src)?;
                auto_detect_unknown_fmt(&cfg_str)
            }
            x => x,
        },
    };

    // If the determined format is "bson" and the `feature` "bson" is enabled, the `cfg_str` is set to "bson".
    // In fact, the "bson" string here only serves as a `placeholder` to prevent the cfg_str corresponding to bson from being empty, and it is not needed at all.
    // Since bson is binary data, you cannot use `read_to_string()`
    #[cfg(feature = "bson")]
    if src_fmt == "bson" {
        cfg_str = "bson".to_owned();
    };

    // If `cfg_str` is still empty, it reads the contents of the input source file into `cfg_str`.
    if cfg_str.is_empty() {
        cfg_str = fs::read_to_string(src)?;
    }

    // It Creates an empty table and sets the table headers using the `set_header` function.
    let mut table = Table::new();
    set_header(
        &mut table,
        &[
            key,
            //
            get_set_text("type").unwrap_or("Type"),
            "Value",
        ],
        table_style,
    );

    // It logs the determined format of the input source file.
    log::info!("{}: {}", get_l10n_text().get("get", "src-fmt")?, src_fmt);

    // A closure function `walk` is defined, which takes in a string `s`. The closure deserialises the TOML data from `s` using `deser_toml`, and uses `walk_toml` to traverse the TOML data and add its key-value pairs to the table. If any key-value pairs are added to the table, it prints the table.
    let mut walk = |s| -> Result<()> {
        // if walk_toml(&deser_toml(s)?, &mut table, key) {
        if walk_cfg(&deser_yaml(s)?, &mut table, key) {
            println!("{table}")
        }

        Ok(())
    };

    // It returns a `ConvFmt` object. This object is used to convert the input data format to TOML.
    let conv = || ConvFmt::new(src, src_fmt, "yaml", Path::new(""), false);

    // It takes in a boolean value `b` and a `high_light_resource` object. The function converts the input data format to TOML using the `convert_data_format` function with the `ConvFmt` object generated by `conv`.
    // If `b` is `true`, it saves the converted TOML data to `cfg_str`.
    let show_highlight_and_save =
        |b| convert_data_format(conv(), b, high_light_resource);

    // It calls `show_highlight_and_save` with `false` as its argument and discards the returned `Result`.
    let just_show_high_light = || show_highlight_and_save(false).map(|_| ());

    // It creates a new `ConvertedContents` object with `cfg_str` and `src_fmt` as its fields.
    let conv_contents = || ConvertedContents::new(cfg_str, src_fmt);

    // Based on the determined input format, the function either traverses the TOML data and prints the resulting table
    // or converts the input data to TOML and saves the resulting TOML data to `cfg_str`.
    // If `key` is empty, it only highlights and shows the input data without traversing it or converting it.
    // Finally, it returns a `Result` object containing a `ConvertedContents` object.
    match src_fmt {
        "yaml" => {
            if key.trim().is_empty() {
                just_show_high_light()?;
                return Ok(conv_contents());
            }
            let res = conv_contents();
            walk(res.get_contents())?;
            Ok(res)
        }
        _ => {
            if key.trim().is_empty() {
                log::info!("{}: yaml", get_l10n_text().get("get", "dst-fmt")?);
                just_show_high_light()?;
                return Ok(conv_contents());
            }

            cfg_str = show_highlight_and_save(true)?.unwrap_or_else(|| {
                panic!("Text content inside {} not obtained", src.display())
            });

            walk(&cfg_str)?;
            Ok(ConvertedContents::new(cfg_str, src_fmt))
        }
    }
}

fn ron_value_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.into(),
        Value::Bool(s) => s.to_string(),
        Value::Mapping(s) => format!("{:?}", s),
        Value::Number(s) => s.to_string(),
        Value::Null => "null".to_owned(),
        Value::Sequence(s) => format!("{:?}", s),
        Value::Tagged(s) => format!("{:?}", s),
        // Value::Unit => "()".to_owned(),
    }
}

fn get_ron_value_type_name(v: &Value) -> &str {
    match v {
        Value::String(_) => "str",
        Value::Bool(_) => "bool",
        // Value::Char(_) => "char",
        Value::Number(s) => match s {
            n if n.is_u64() => "u64",
            n if n.is_i64() => "i64",
            n if n.is_nan() => "NaN",
            n if n.is_infinite() => "Inf",
            // n if n.is_f64() => "f64",
            _ => "f64",
        },
        Value::Sequence(_) => "array",
        Value::Mapping(_) => "map",
        Value::Tagged(_) => "enum",
        _ => "Null",
        // Value::Option(_) => "Option<Value>",
        // Value::Unit => "Unit",
    }
}
/// Used to traverse the config data in a breadth-first search manner.
///
/// It takes in a value, a table, and a query string as its arguments.
/// It checks if each key matches the query and adds its key-value pairs to the table if they match or continue traversing if they don't match.
/// If it encounters an array or another table, it adds their elements or key-value pairs to the deque respectively.
/// If a leaf value matches the query, it prints the value and returns false to stop the traversal.
pub fn walk_cfg(value: &Value, table: &mut Table, query: &str) -> bool {
    use std::collections::VecDeque;

    let mut deque: VecDeque<(String, &Value)> = VecDeque::with_capacity(128);

    deque.push_back((String::new(), value));

    let query = query.trim_start_matches(['.', ' ']);
    // let prefix = query
    //     .contains('.')
    //     .then_some(query);
    let prefix = Some(query);

    let mut key_dot = String::with_capacity(16);
    let empty = |s: &str| s.is_empty();

    while let Some((key, v)) = deque.pop_front() {
        match v {
            Value::Mapping(table) => {
                // If we encounter a table, add its key-value pairs to the front of the deque.
                for (k, v) in table.iter() {
                    key_dot = ron_value_to_string(k);

                    let new_key = match key.trim() {
                        t if empty(t) && key_dot.contains('.') => {
                            format!(r#"'{}'"#, key_dot)
                        }
                        t if empty(t) => key_dot.to_owned(),
                        _ if key_dot.contains('.') => {
                            format!(r#"{}.'{}'"#, key, key_dot)
                        }
                        _ => format!(r#"{}.{}"#, key, key_dot),
                    };
                    deque.push_back((new_key, v));
                }
            }
            Value::Sequence(array) => {
                // If we encounter an array, add its elements to the front of the deque with their index as a suffix to the key.
                for (i, v) in array.iter().enumerate() {
                    let new_key = format!("{}.{}", key, i);
                    deque.push_back((new_key, v));
                }
            }
            _ => {
                // If we encounter a leaf value, check if its key matches the query.
                // If it matches, print the value and return false to stop the traversal.
                // If it doesn't match, add its key-value pair to the table.
                if !query_key_and_push_table(
                    &key,
                    query,
                    &mut key_dot,
                    prefix,
                    v,
                    table,
                ) {
                    return false;
                }
            }
        }
    }
    // Returns true if at least one row was added to the table.
    matches!(table.row(0), Some(_))
}

/// A helper function that takes in a key, query string, prefix, and value as its arguments.
/// It checks if the given key matches the query, and adds its key-value pair to the table if it does.
/// It also handles cases where the key has the same prefix as the query or doesn't match the query at all.
///
/// If false is returned, the table is empty.
fn query_key_and_push_table(
    key: &str,
    query: &str,
    key_dot: &mut String,
    prefix: Option<&str>,
    v: &Value,
    table: &mut Table,
) -> bool {
    // If there is no match, then return true.
    if !key.starts_with(query) {
        // Note: Please do not return false!
        // In the `walk_toml()` loop matching value, if false is returned it will terminate the loop.
        return true;
    }

    *key_dot = format!("{query}.");
    let new_prefix = match key {
        k if k.starts_with(&*key_dot) => Some(key_dot.as_str()),
        _ => prefix,
    };

    let v_str = ron_value_to_string(v);
    let v_type = get_ron_value_type_name(v);

    match &new_prefix {
        // Check if the current key is the same as the query.
        // If it is, print the value and return false to stop the traversal.
        Some(s) if s == &key => {
            info!(
                "{}: {:?}",
                get_set_text("type")
                    .unwrap_or("Type")
                    .cyan(),
                v_type.purple()
            );
            info!("{}: {:?}", "value".cyan(), v_str.blue());
            println!("{}", v_str);
            // match v {
            //     Value::String(s) => println!("{s}"),
            //     _ => println!("{v}"),
            // }
            return false;
        }
        // If the current key has the same prefix as the query, add its (trimmed-key, value) pair to the table.
        // If not, add the full key-value pair to the table.
        Some(s) => {
            log::trace!("key: {}, s: {}", key, s);
            let k = if s.contains('.') {
                key.trim_start_matches(s)
            } else {
                table.set_header([
                    ".",
                    get_set_text("type").unwrap_or("Type"),
                    "Value",
                ]);
                key
            };
            table.add_row([k.trim_start_matches('.'), v_type, &v_str]);
        }
        _ => {
            table.add_row([key, v_type, &v_str]);
        }
    }
    true
}
