use crate::{
    assets::get_l10n_text,
    conversion::{
        convert::convert_data_format,
        // fmt::json5_as_json,
        get_conv_dft, get_dst, get_dst_fmt, get_static_stdin_data, is_src_stdin,
        serialisation::{deser_toml_to_yaml, deser_yaml, ser_toml},
        ConvFmt,
    },
    get::{get_config_file, walk_cfg, ConvertedContents},
    highlight::{output::get_syntax_highlight, HighLightRes},
    table::set_header,
};
use anyhow::Result;
use getset::{Getters, MutGetters};
use glossa::GetText;
use log::{debug, info, warn};
use owo_colors::OwoColorize;
use std::{
    borrow::Cow,
    fs, io,
    path::{Path, PathBuf},
};
pub use toml_edit::{
    Array, ArrayOfTables, Datetime, InlineTable, Table as TomlTable, Value,
};
use toml_edit::{Document, Item};

/// Used to get the mutable Item in toml Document.
///
/// The function takes in a mutable reference to a Document and an iterator of string slices as inputs, and returns a mutable reference to an Item.
fn get_item_mut<'doc, 's, I: IntoIterator<Item = &'s str>>(
    doc: &'doc mut Document,
    keys: I,
) -> &'doc mut Item {
    let mut tmp = doc.as_item_mut(); // Initialise it to the mutable reference obtained using 'as_item_mut' method on the given 'doc' object of Document type.
    let mut is_arr = false;

    for key in keys.into_iter() {
        // Iterate over the keys provided as input.
        if key.is_empty() {
            // Check if the current key is empty. If yes, return the current item.
            return tmp;
        }
        tmp = match is_arr {
            // Match the value of 'is_arr'.
            true => tmp.get_mut(
                // If it is true, consider that the previous key got an array item so obtain its index(i.e. the index of the array).
                key.parse::<usize>() // Parse the string key to an unsigned integer value of type usize.
                    .expect("Not a Arr"),
            ),
            _ => tmp.get_mut(key),
        }
        .unwrap_or_else(|| panic!("Failed to get: {}", key)); // Panic with a custom error message if the child item isn't found.

        is_arr = matches!(tmp.type_name(), "array" | "array of tables"); // Check if the current item is of array type. If yes, set 'is_arr' to true to process its elements in the next iteration.
    }
    tmp // Return the final item found.
}

#[derive(Getters, MutGetters, Debug)]
#[getset(get = "pub with_prefix", get_mut = "pub with_prefix")]
pub struct CfgOpts<'s, 'd> {
    src: &'s Path,
    src_format: Option<&'s String>,
    save: bool,
    dst: Option<&'d PathBuf>,
    key: TomlKey<'s>,
    value: TomlValue,
    high_light: Option<&'d HighLightRes<'d>>,
    table_style: &'d str,
    set_value: bool,
    preview: bool,
}

/// This enum could be used to represent a key in a TOML document, where the key can either be a simple string or a sequence of strings representing a nested structure of keys.
///
/// For Str("a.b".c), it will be parsed as ["a", "b", "c"],
/// while Vec(["a.b", "c"]) would be ["a.b", "c"]
///
/// - `Str`: it contains a `Cow` object, which is a smart pointer that can represent either an owned (`Owned`) or a borrowed (`Borrowed`) value. In this case, the `Cow` object holds a borrowed string with lifetime `'s`.
/// - `Vec`: it contains a reference to a vector of strings, also with lifetime `'s`.
///
/// The enum definition specifies that both variants have a lifetime parameter `'s`, which means that any references contained within them cannot outlive the `TomlKey` object itself.
#[derive(Debug, Clone)]
pub enum TomlKey<'s> {
    Str(Cow<'s, str>),
    Vec(&'s Vec<String>),
}

impl<'s> TomlKey<'s> {
    /// The `match` expression matches on the variant of the `TomlKey` enum.
    ///
    /// If it's `TomlKey::Str`, then the string key is cloned and returned directly.
    ///
    /// If instead the `TomlKey` variant is `TomlKey::Vec`, then each element of the vector is filtered to exclude any empty strings, then the `quote_dot` function is applied to each non-empty string.
    ///
    /// `TomlKey::Vec(v)` is related to `--concat-key` of `tomlyre-cli`.
    /// If the user enters a key that contains `.`, then it will automatically add quotes to the key.
    /// When matching table data, keys with `.` must be quoted, otherwise it will be treated as a child table.
    /// Next, all valid elements are concatenated (using `.` to concatenate) into a string.
    ///
    /// This concatenated string is then wrapped in a `Cow` (short for "clone-on-write") type, which allows the string to be returned either by reference or by clone depending on whether it has already been borrowed elsewhere in the program.
    pub fn concat_key(&self) -> Cow<str> {
        match self {
            TomlKey::Str(k) => k.clone(),
            TomlKey::Vec(v) => {
                let s = v
                    .iter()
                    .filter(|x| !x.is_empty())
                    .map(|x| quote_dot(x))
                    // .map(|x| x.as_str())
                    .collect::<Vec<_>>()
                    .join(".");
                Cow::from(s)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TomlValue {
    Normal,
    Table(toml_edit::Table),
    AOT(toml_edit::ArrayOfTables),
    NONE,
}

impl<'s, 'd> CfgOpts<'s, 'd> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        src: &'s Path,
        src_format: Option<&'s String>,
        save: bool,
        dst: Option<&'d PathBuf>,
        key: TomlKey<'s>,
        high_light: Option<&'d HighLightRes<'d>>,
        table_style: &'d str,
    ) -> Self {
        Self {
            src,
            src_format,
            save,
            dst,
            key,
            value: TomlValue::Normal,
            high_light,
            table_style,
            set_value: true,
            preview: false,
        }
    }
}

/// This is a Rust function called `quote_dot` that takes a reference to a string (`&str`) as its argument and returns a string.
///
/// The purpose of this function is to quote a string with single quotes if it contains a dot (".") character. If the input string contains no dots, it returns a copy of the input string.
///
/// Here's how it works:
/// 1. The function first checks whether the input string contains a dot character by calling the `contains` method on the input string.
/// 2. If the input string contains a dot, the function formats a new string using the `format!` macro. The format string includes single quotes around the original string, so it effectively quotes the input string. The resulting quoted string is returned from the function.
/// 3. If the input string does not contain a dot, the function simply calls the `to_owned` method on the input string to create a new owned copy of the string, which is then returned from the function.
fn quote_dot(x: &str) -> String {
    if x.contains('.') {
        format!("'{}'", x)
    } else {
        x.to_owned()
    }
}

impl<'s, 'd> CfgOpts<'s, 'd> {
    /// The purpose of this method is to retrieve the current value of a given configuration key from a configuration file.
    ///
    /// It does this by calling the get_config_file function with several arguments, including the key, the source file path, the source format (if applicable), and some highlighting and table formatting options.
    /// If the call succeeds and returns a ConvertedContents struct, the method returns a tuple containing the contents of the configuration value as a string, and the format of the configuration file.
    fn get_current_cfg_value(&self, key: &str) -> Result<(String, &str)> {
        log::info!("Current:");

        let ConvertedContents { contents, fmt } = get_config_file(
            key,
            self.src,
            self.src_format,
            self.high_light,
            self.table_style,
        )?;

        Ok((contents, fmt))
    }

    pub fn set_value<V: Into<Value>>(&self, s: Option<V>) -> Result<()> {
        let key = self.get_concatenated_key_str();
        debug!("key: {key}");

        let (mut cfg_str, src_fmt) = self.get_current_cfg_value(&key)?;

        match self.get_key() {
            // If the key is an empty string or a single period character, immediately return an Ok result containing unit ().
            TomlKey::Str(k) if k == "." || k.trim().is_empty() => return Ok(()),
            // If the key is a vector containing an empty string, a single period character, or has no elements, immediately return an Ok result containing unit ().
            TomlKey::Vec(k)
                if k.is_empty() || k[0] == "." || k[0].trim().is_empty() =>
            {
                return Ok(())
            }
            _ => {}
        }

        // Call the reread_toml_str method with src_fmt and a mutable reference to cfg_str.
        // The reason for re-reading toml is to preserve the comments.
        self.reread_toml_str(src_fmt, &mut cfg_str)?;

        // Parse cfg_str as a TOML document using the parse method of the Document type.
        let mut doc = cfg_str.parse::<Document>()?;

        let key_chunks = self.get_key_chunks();
        // Get a mutable reference to the value at those key chunks in the TOML document by calling the get_item_mut function with an iterator over the copied key chunks.
        let v = get_item_mut(&mut doc, key_chunks.iter().copied());

        // If self.set_value is false, call the print_current_kv method with v and a reference to key_chunks.
        if !self.set_value {
            self.print_current_kv(v, &key_chunks)?;
            return Ok(());
        }
        // It sets the value at the specified location in the TOML document.
        self.set_toml_value(s, v);

        // Log a message to the console that includes the key_chunks, the type name of v, and the new value of v.
        println!(
            "\nkey:\t{:?}\ntype:\t{}\nnew value: {}\n",
            key_chunks.cyan(),
            v.type_name().blue(),
            v.yellow().bold(),
        );

        // Convert the modified TOML document back into a string using the to_string method of doc.
        let modified_str = doc.to_string();

        if key_chunks.len() >= 2 {
            self.print_new_value_table(key_chunks, &modified_str)?;
        }
        let mut converted = None;

        #[allow(unused_assignments)]
        let mut dst_fmt_lcase = String::with_capacity(src_fmt.len() + 5);

        // Get the final destination path
        let (dst_fmt, dst) = match (self.get_dst_path(), self.get_src()) {
            (d, s) if &d == s => (src_fmt, d),
            (d, ..) => {
                // If dst_fmt_lcase is a valid format, assign it to dst_fmt, or else use src_fmt.
                dst_fmt_lcase = d
                    .file_name()
                    .unwrap_or_else(|| {
                        panic!("{} {}", get_set_dft("invalid-path"), d.display())
                    })
                    .to_string_lossy()
                    .to_ascii_lowercase();

                let fmt = 
                // json5_as_json(
                    get_dst_fmt(&dst_fmt_lcase, false).unwrap_or(src_fmt)
                // )
                ;

                let dst = get_dst(&d, fmt, self.get_src());
                (fmt, dst)
            }
        };

        // If preview mode is on, check the file type of dst_fmt. If it is TOML, set converted to modified_str, else call the `convert_or_get_str()` method to get the converted string.
        if *self.get_preview() {
            match dst_fmt {
                "toml" => converted = Some(modified_str.to_string()),
                "bson" => {
                    get_syntax_highlight(
                        "toml",
                        &modified_str,
                        self.high_light,
                        None,
                    )?;
                    log::info!("Bson: Binary data");
                }
                _ => {
                    // Assign the resulting string to converted.
                    converted = self.convert_or_get_str(
                        dst_fmt,
                        Path::new(""),
                        &modified_str,
                        true,
                    )?
                }
            }
            if let Some(ref s) = converted {
                get_syntax_highlight(dst_fmt, s, self.high_light, None)?;
            }
            println!();
        }

        self.show_unsave_warning(&dst);

        // If save mode is on, check if converted contains a string.
        // If it does, log an info message and write the string to the file at dst.
        // If it doesn't, call the `convert_or_get_str()` to convert and write to the file at dst.
        if *self.get_save() {
            match converted {
                Some(ref s) => {
                    info!("Saved at: {}", dst.display());
                    fs::write(dst, s)?;
                }
                _ => {
                    self.convert_or_get_str(dst_fmt, &dst, &modified_str, false)?;
                }
            }
        }

        Ok(())
    }

    /// Updates the toml value according to the data type.
    ///
    /// The function checks if the optional argument `s` is some value. If it is, the function uses the `toml_edit::value` function to convert the value into a TOML value and update the `Item` object's value.
    /// If `s` is None, the function checks the value of `self.get_value()`. Depending on the type of value returned by `get_value()`, the `Item` object's value is updated accordingly:
    ///
    /// - If the value is `Normal`, do nothing.
    /// - If the value is a `Table`, update the `Item` object's value to a new `Table` with the same content as the original one.
    /// - If the value is an `AOT` (array of tables), update the `Item` object's value to a new `ArrayOfTables` with the same content as the original one.
    /// - If the value is `NONE`, update the `Item` object's value to `None`.This is equivalent to deleting the key.
    fn set_toml_value<V: Into<Value>>(&self, s: Option<V>, v: &mut Item) {
        use TomlValue::*;
        match s {
            Some(s) => *v = toml_edit::value::<V>(s),
            _ => match self.get_value() {
                Normal => {}
                Table(t) => *v = Item::Table(t.to_owned()),
                AOT(t) => *v = Item::ArrayOfTables(t.to_owned()),
                NONE => *v = Item::None,
            },
        }
    }

    /// If the data was previously fetched via `get`, then the original string of `toml` needs to be re-read in order to preserve the comments when `set`.
    ///
    /// For non-toml formats, since tomlyre does not support reserved comments, it is sufficient to use the string previously retrieved from `get`.
    ///
    /// The function first uses a `match` expression to check if the value of `src_fmt` is equal to the string "toml" and if the source is from standard input (determined by the `is_src_stdin` function).
    ///
    /// If this is true, it calls the `get_static_stdin_data` function to obtain static data from standard input and assigns it to `cfg_str` using the dereference operator (`*`) and the `to_owned` method.
    ///
    /// If `src_fmt` is equal to "toml" but the source is not from standard input, the function uses the `fs::read_to_string` function to read the contents of the file specified by `self.get_src()` into `cfg_str`.
    ///
    /// If `src_fmt` is not equal to "toml", then yaml -> toml.
    fn reread_toml_str(&self, src_fmt: &str, cfg_str: &mut String) -> Result<()> {
        match src_fmt {
            "toml" if is_src_stdin(self.get_src()) => {
                *cfg_str = get_static_stdin_data().to_owned()
            }
            "toml" => *cfg_str = fs::read_to_string(self.get_src())?,
            _ => *cfg_str = ser_toml(&deser_yaml(cfg_str)?)?,
        };
        Ok(())
    }

    /// Returns a concatenated string representation of a key, which may consist of multiple components.
    fn get_concatenated_key_str(&self) -> Cow<str> {
        self.get_key().concat_key()
    }

    /// Gets the key by `get_key()` and split or filter it according to the type of `TomlKey` to generate a valid `Vec<key>`
    ///
    /// The `match` expression matches on the variant of the `TomlKey` enum returned by `get_key()`. If it's `TomlKey::Str`, then the key is split into components using the period (`.`) separator, and any empty components are filtered out. The resulting non-empty components are collected into a vector of string slices.
    ///
    /// If the `TomlKey` variant is `TomlKey::Vec`, then each element of the vector is mapped to its string representation via the `as_str()` method, and any empty strings are filtered out. The resulting non-empty strings are collected into a vector of string slices.
    ///
    /// Finally, the resulting vector of string slices is returned as the output of this function.
    fn get_key_chunks(&self) -> Vec<&str> {
        match self.get_key() {
            TomlKey::Str(s) => s
                .split('.')
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>(),
            TomlKey::Vec(v) => v
                .iter()
                .map(|x| x.as_str())
                .filter(|x| !x.is_empty())
                .collect(),
        }
    }

    /// Performs syntax highlighting on the TOML-formatted string representation of the Item object's value.
    fn print_current_kv(
        &self,
        v: &mut Item,
        key_chunks: &Vec<&str>,
    ) -> Result<(), anyhow::Error> {
        print!(
            "\nkey:\t{:?}\ntype:\t{}",
            key_chunks.cyan(),
            v.type_name().blue().bold(),
        );
        println!("\nvalue: ");
        get_syntax_highlight(
            "toml",
            &v.to_string(),
            self.get_high_light().as_deref(),
            None,
        )?;
        println!();
        Ok(())
    }

    /// Provides a convenient way to visualize changes made to a configuration key using a table format.
    ///
    /// | Parameter       | Description                                                                           |
    /// | ----------------| ------------------------------------------------------------------------------------- |
    /// | `mut key_chunks`| a mutable vector of string slices, representing the components of a configuration key |
    /// | `modified_str`  | a string containing the modified configuration value.                                 |
    ///
    /// The purpose of this method is to print a table showing the new value of a configuration key after it has been modified. It does this by parsing the modified configuration value into a TOML object using the `deser_toml` function, and then walking through the TOML object to generate a table using the `comfy-table` crate.
    ///
    /// The method first initialises an empty `Table` object from `comfy-table`, and defines a closure `set_table` to set the header for the table. The `key_chunks` argument is used to build the header of the table.
    ///
    /// The method then pops the last element of the `key_chunks` vector (which represents the modified key), and constructs a "dotted" version of the remaining key components using the `quote_dot` function. This dotted key is passed to `set_table` to set the header of the table.
    ///
    /// Next, the method calls the `walk_toml` function with the parsed TOML object, the `Table` object, and the "dotted" key. If `walk_toml` returns `true`, indicating that the table was successfully generated, the method prints the resulting table to the console using `println!`.
    ///
    /// Finally, the method returns an `Ok` result.
    fn print_new_value_table(
        &self,
        mut key_chunks: Vec<&str>,
        modified_str: &str,
    ) -> Result<(), anyhow::Error> {
        let mut table = comfy_table::Table::new();
        let mut set_table = |k| {
            set_header(
                &mut table,
                &[
                    k,
                    get_set_text("type").unwrap_or("Type"),
                    get_set_dft("new-value").as_ref(),
                ],
                self.table_style,
            )
        };
        key_chunks.pop();

        let chunks_no_last = key_chunks
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| quote_dot(x))
            .collect::<Vec<_>>()
            .join(".");
        set_table(&chunks_no_last);

        if walk_cfg(
            &deser_toml_to_yaml(modified_str)?,
            &mut table,
            &chunks_no_last,
        ) {
            println!("{table}")
        };
        Ok(())
    }

    /// Computes the destination file path where changes made to a configuration file should be saved.
    ///
    /// It does this by inspecting several fields in the struct:
    ///
    /// - `dst`: the user-specified destination file path, if any.
    /// - `src`: the source file path where the configuration file is located.
    ///
    /// The method uses a `match` to check whether `dst` is `Some(p)` (i.e., a valid user-specified path) and whether it is a directory. If `dst` is a directory, the method constructs a new path by joining the directory with the filename of the source file, using the `join` method on the `PathBuf` object returned by `p`. If `dst` is not a directory, the method simply converts it into a `PathBuf` object using the `into` method.
    ///
    /// If `self.dst` is `None`, indicating that the user did not specify a destination path, the method returns a `PathBuf` object representing the source path.
    ///
    /// This function is related to `--save` and `--to` of `tomlyre-cli`, and if the user does not specify `--to`, then dst becomes the src path and overwrites the source file when the user calls `--save`.
    fn get_dst_path(&self) -> PathBuf {
        match self.dst {
            Some(p) if p.is_dir() => p.join(
                self.get_src()
                    .file_name()
                    .expect("Failed to get src fname"),
            ),
            Some(p) => p.into(),
            _ => self.get_src().into(),
        }
    }

    /// Converts a `non-toml` format to the corresponding `dst_fmt`, and returns the string as required.
    ///
    /// > In `tomlyre-cli`, it is necessary to return the converted string if `--preview` is called during `set`.
    ///
    /// The `dst_fmt` parameter is a string slice that represents the desired output format. The function uses a match expression to determine what action to take based on this value.
    ///
    /// If `dst_fmt` is "toml", the function writes the contents of `modified_str` to the file at the path specified by `dst`, using the `fs::write` function.
    ///
    /// If `dst_fmt` is any other value, the function creates a temporary file with a name that includes the filename of the source file being converted (if it exists), and writes the contents of `modified_str` to it using `fs::write`.
    /// It then creates a `ConvFmt` struct with the paths to the temporary file
    /// - the desired input format ("toml")
    /// - the desired output format (`dst_fmt`)
    /// - the destination path (`dst`)
    /// - and a boolean flag indicating whether or not to save the file.
    ///
    /// The function then calls the `convert_data_format` function on this `ConvFmt` struct, passing in true if `get_str` is true (indicating that the function should return the converted data as a string), and false otherwise.
    ///
    /// If `get_str` is true, the function removes the temporary file and returns the converted data as an `Ok(Some(String))` value.
    /// Otherwise, it simply removes the temporary file and returns an `Ok(None)` value.
    ///
    /// The `modified_str` parameter is a string slice containing the contents of the source file after some modifications have been made.
    ///
    /// The `dst` parameter is a `Path` representing the path to the destination file.
    ///
    /// The `get_str` parameter is a boolean flag indicating whether or not the function should return the converted data as a string.
    ///
    /// The function returns a `Result<Option<String>>` value, where the `Ok` variant contains either `Some(String)` if `get_str` is true and the conversion was successful, or `None` otherwise. If an error occurs during the execution of the function, the `Err` variant will contain the error.
    fn convert_or_get_str(
        &self,
        dst_fmt: &str,
        dst: &Path,
        modified_str: &str,
        get_str: bool,
    ) -> Result<Option<String>> {
        match dst_fmt {
            "toml" => fs::write(dst, modified_str)?,
            _ => {
                let tmp = dst.parent().unwrap_or_else(|| Path::new(".")).join(format!(
                    "._{}.tomlyre.QeAlU63e0qGhBdCpYhNm9ZNGNq0RVaxZ5ytDBE0eGiLVFK97UPDXkyO5RCNaVFdk.tmp",
                    self.get_src().file_name().unwrap_or_default().to_string_lossy()
                ));

                fs::write(&tmp, modified_str)?;
                let conv =
                    ConvFmt::new(&tmp, "toml", dst_fmt, dst, *self.get_save());
                let high_light = self.get_high_light();

                let rm_tmp = || -> io::Result<()> {
                    fs::remove_file(&tmp)?;
                    Ok(())
                };

                if get_str {
                    let str = convert_data_format(conv, true, *high_light)?;
                    rm_tmp()?;
                    return Ok(str);
                }

                convert_data_format(conv, false, *high_light)?;
                rm_tmp()?;
            }
        };
        Ok(None)
    }

    /// The purpose of this method is to warn the user that changes they made will not be saved to the destination file path specified by dst.
    ///
    /// It does this by checking whether the save field in the struct is set to true, and returning early if it is.
    /// If save is false, the method logs a warning message using the warn! macro.
    ///
    /// The warning message includes the following information:
    ///
    /// - The user has not called the `--save` option.
    /// - The location where changes would normally be saved, specified by dst.
    fn show_unsave_warning(&self, dst: &Path) {
        if *self.get_save() {
            return;
        }
        warn!("{}", get_set_md("unsave-warn"));
        info!("{}: {:?}", get_conv_dft("dst"), dst.as_os_str().blue());
    }
}

fn get_set_dft(k: &str) -> Cow<str> {
    get_l10n_text().get_or_default("set", k)
}

fn get_set_md(k: &str) -> Cow<str> {
    get_l10n_text().get_or_default("set_md", k)
}

pub(crate) fn get_set_text(k: &str) -> glossa::Result<&str> {
    get_l10n_text().get("set", k)
}
