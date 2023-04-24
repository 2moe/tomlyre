use crate::{
    conversion::{
        fmt::FORMATS,
        get_conv_md, get_conv_text, get_static_stdin_data, is_src_stdin,
        serialisation::{deser_toml, ser_toml},
        ConvFmt,
    },
    highlight::{output::get_syntax_highlight, HighLightRes},
};
use log::warn;
use owo_colors::OwoColorize;
use std::{borrow::Cow, fs, io};

#[cfg(feature = "xml")]
use crate::conversion::serialisation::{deser_xml, ser_xml};

#[cfg(feature = "lexpr")]
use crate::conversion::serialisation::ser_lexpr;

#[cfg(feature = "bson")]
use crate::conversion::serialisation::{deser_bson, ser_bson_to_bin};

#[cfg(feature = "json")]
use crate::conversion::serialisation::{deser_json, ser_json};

#[cfg(all(feature = "json5", feature = "json"))]
use crate::conversion::serialisation::deser_json5;

#[cfg(feature = "json5")]
use crate::conversion::serialisation::ser_json5;

#[cfg(feature = "ron")]
use crate::conversion::serialisation::{deser_ron, ser_ron};

#[cfg(feature = "yaml")]
use crate::conversion::serialisation::{deser_yaml, ser_yaml};

pub(crate) fn convert_data_format(
    conv: ConvFmt,
    to_string: bool,
    high_light_style: Option<&HighLightRes>,
) -> anyhow::Result<Option<String>> {
    // Uses Rust's destructuring syntax to extract values from the ConvFmt struct passed as an argument to the function.
    let ConvFmt {
        src,
        src_fmt,
        dst_fmt,
        dst,
        save,
    } = conv;

    // Reads the contents of the input file based on its format. If it is BSON, an empty string is returned. If the data comes from stdin, this function returns data from a static buffer. If the data comes from a file, Rust's fs library reads and returns the contents of the file.
    let s = match src_fmt {
        "bson" => Cow::from(""),
        _ if is_src_stdin(&src) => Cow::from(get_static_stdin_data()),
        _ => Cow::from(fs::read_to_string(src.as_ref())?),
    };

    // Checks for configuration flags and if the input format is JSON, it checks whether the data is valid JSON 1.0 format or JSON5 format. If the format is invalid, a warning message is issued, and JSON5 is attempted instead.
    #[cfg(all(feature = "json5", feature = "json"))]
    let src_fmt = match src_fmt {
        "json" => {
            if deser_json(&s).is_ok() {
                "json"
            } else {
                warn!("{}", get_conv_md("invalid-json1.0")?);
                "json5"
            }
        }
        _ => src_fmt,
    };

    // Used to check whether a dst path value is empty or not.(Path::new("   ") & Path::new("") are both empty.)
    let is_not_empty = |o: &str| !o.trim().is_empty();

    match (save, dst.as_os_str().to_str()) {
        (false, Some(o)) if is_not_empty(o) => {
            log::info!("{}: {}", get_conv_text("dst")?, dst.display().yellow());
            warn!("{}", get_conv_md("not-saved")?);
        }
        (true, Some(o)) if is_not_empty(o) => {
            log::info!("Saving to {}", dst.display())
        }
        _ => {}
    }

    // Takes in contents (a String) as input and returns an `io::Result<Option<String>>`. This closure is responsible for writing the converted data to the appropriate output format or file.
    let to_dst = |contents: String| -> io::Result<Option<String>> {
        // If the save flag is set and a valid output path is specified, the closure uses Rust's fs library to write the contents of the contents variable to the destination file. In this case, the closure returns Ok(None) since the data has been successfully saved to file.
        if save && !dst.as_os_str().is_empty() {
            fs::write(&dst, contents)?;
            return Ok(None);
        }

        // If the to_string flag is set, the function returns Ok(Some(contents)), indicating that the converted data should be returned as a string.
        if to_string {
            return Ok(Some(contents));
        }

        get_syntax_highlight(dst_fmt, &contents, high_light_style, None)?;

        match dst_fmt {
            // print "\n"
            "json" | "json5" | "ron" | "sexp" | "xml" => println!(),
            _ => {}
        }
        Ok(None)
    };

    // Takes a slice of bytes (`&[u8]`) called `bin` as input and returns an I/O result containing either a `String` or `None`.
    //
    // The closure first checks if the boolean variable `save` is true. If so, it writes the binary data to a file specified by the string variable `dst` using the `fs::write` function and returns `None` in the I/O result.
    //
    // If `save` is false, the closure simply prints a message to the console indicating that the data is binary and specifying the path to the file where it would have been saved. It then returns `None` in the I/O result.
    #[cfg(feature = "bson")]
    let write_bin = |bin: &[u8]| -> io::Result<Option<String>> {
        if save {
            fs::write(&dst, bin)?;
        } else {
            println!("binary data, {}", dst.display());
        }
        Ok(None)
    };

    let opt_str = match (src_fmt, dst_fmt) {
        #[cfg(feature = "yaml")]
        ("toml", "yaml") => to_dst(ser_yaml(&deser_toml(&s)?)?)?,
        #[cfg(feature = "json")]
        ("toml", "json") => to_dst(ser_json(&deser_toml(&s)?)?)?,
        #[cfg(feature = "ron")]
        ("toml", "ron") => to_dst(ser_ron(&deser_toml(&s)?)?)?,
        #[cfg(feature = "bson")]
        ("toml", "bson") => write_bin(&ser_bson_to_bin(&deser_toml(&s)?)?)?,
        #[cfg(feature = "lexpr")]
        ("toml", "sexp") => to_dst(ser_lexpr(&deser_toml(&s)?)?)?,
        ("toml", "toml") => to_dst(ser_toml(&deser_toml(&s)?)?)?,
        #[cfg(feature = "xml")]
        ("toml", "xml") => to_dst(ser_xml(&deser_toml(&s)?)?)?,
        #[cfg(feature = "json5")]
        ("toml", "json5") => to_dst(ser_json5(&deser_toml(&s)?)?)?,

        #[cfg(all(feature = "yaml"))]
        ("yaml", "toml") => to_dst(ser_toml(&deser_yaml(&s)?)?)?,
        #[cfg(feature = "yaml")]
        ("yaml", "yaml") => to_dst(ser_yaml(&deser_yaml(&s)?)?)?,
        #[cfg(all(feature = "yaml", feature = "json"))]
        ("yaml", "json") => to_dst(ser_json(&deser_yaml(&s)?)?)?,
        #[cfg(all(feature = "yaml", feature = "ron"))]
        ("yaml", "ron") => to_dst(ser_ron(&deser_yaml(&s)?)?)?,
        #[cfg(all(feature = "yaml", feature = "bson"))]
        ("yaml", "bson") => write_bin(&ser_bson_to_bin(&deser_yaml(&s)?)?)?,
        #[cfg(all(feature = "yaml", feature = "lexpr"))]
        ("yaml", "sexp") => to_dst(ser_lexpr(&deser_yaml(&s)?)?)?,
        #[cfg(all(feature = "yaml", feature = "xml"))]
        ("yaml", "xml") => to_dst(ser_xml(&deser_yaml(&s)?)?)?,
        #[cfg(all(feature = "yaml", feature = "json5"))]
        ("yaml", "json5") => to_dst(ser_json5(&deser_yaml(&s)?)?)?,

        #[cfg(feature = "json")]
        ("json", "toml") => to_dst(ser_toml(&deser_json(&s)?)?)?,
        #[cfg(feature = "json")]
        ("json", "json") => to_dst(ser_json(&deser_json(&s)?)?)?,
        #[cfg(all(feature = "json", feature = "yaml"))]
        ("json", "yaml") => to_dst(ser_yaml(&deser_json(&s)?)?)?,
        #[cfg(all(feature = "json", feature = "ron"))]
        ("json", "ron") => to_dst(ser_ron(&deser_json(&s)?)?)?,
        #[cfg(all(feature = "json", feature = "bson"))]
        ("json", "bson") => write_bin(&ser_bson_to_bin(&deser_json(&s)?)?)?,
        #[cfg(all(feature = "json", feature = "lexpr"))]
        ("json", "sexp") => to_dst(ser_lexpr(&deser_json(&s)?)?)?,
        #[cfg(all(feature = "json", feature = "xml"))]
        ("json", "xml") => to_dst(ser_xml(&deser_json(&s)?)?)?,
        #[cfg(all(feature = "json", feature = "json5"))]
        ("json", "json5") => to_dst(ser_json5(&deser_json(&s)?)?)?,

        #[cfg(feature = "ron")]
        ("ron", "toml") => to_dst(ser_toml(&deser_ron(&s)?)?)?,
        #[cfg(all(feature = "ron", feature = "yaml"))]
        ("ron", "yaml") => to_dst(ser_yaml(&deser_ron(&s)?)?)?,
        #[cfg(all(feature = "ron", feature = "json"))]
        ("ron", "json") => to_dst(ser_json(&deser_ron(&s)?)?)?,
        #[cfg(all(feature = "ron", feature = "bson"))]
        ("ron", "bson") => write_bin(&ser_bson_to_bin(&deser_ron(&s)?)?)?,
        #[cfg(all(feature = "ron", feature = "lexpr"))]
        ("ron", "sexp") => to_dst(ser_lexpr(&deser_ron(&s)?)?)?,
        #[cfg(feature = "ron")]
        ("ron", "ron") => to_dst(ser_ron(&deser_ron(&s)?)?)?,
        #[cfg(all(feature = "ron", feature = "xml"))]
        ("ron", "xml") => to_dst(ser_xml(&deser_ron(&s)?)?)?,
        #[cfg(all(feature = "ron", feature = "json5"))]
        ("ron", "json5") => to_dst(ser_json5(&deser_ron(&s)?)?)?,

        #[cfg(feature = "bson")]
        ("bson", "toml") => to_dst(ser_toml(&deser_bson(&src)?)?)?,
        #[cfg(all(feature = "yaml", feature = "bson"))]
        ("bson", "yaml") => to_dst(ser_yaml(&deser_bson(&src)?)?)?,
        #[cfg(all(feature = "bson", feature = "json"))]
        ("bson", "json") => to_dst(ser_json(&deser_bson(&src)?)?)?,
        #[cfg(all(feature = "ron", feature = "bson"))]
        ("bson", "ron") => to_dst(ser_ron(&deser_bson(&src)?)?)?,
        #[cfg(all(feature = "lexpr", feature = "bson"))]
        ("bson", "sexp") => to_dst(ser_lexpr(&deser_bson(&src)?)?)?,
        #[cfg(all(feature = "bson", feature = "xml"))]
        ("bson", "xml") => to_dst(ser_xml(&deser_bson(&src)?)?)?,
        #[cfg(all(feature = "bson", feature = "json5"))]
        ("bson", "json5") => to_dst(ser_json5(&deser_bson(&src)?)?)?,
        ("sexp", _) => {
            panic!("{}", get_conv_md("not-support-deser-sexp")?);
        }
        /*
        #[cfg(feature = "lexpr")]
        ("sexp", "toml") => to_dst(ser_toml(&deser_lexpr(&s)?)?)?,
        #[cfg(all(feature = "lexpr", feature = "yaml"))]
        ("sexp", "yaml") => to_dst(ser_yaml(&deser_lexpr(&s)?)?)?,
        #[cfg(all(feature = "lexpr", feature = "json"))]
        ("sexp", "json") => to_dst(ser_json(&deser_lexpr(&s)?)?)?,
        #[cfg(all(feature = "lexpr", feature = "ron"))]
        ("sexp", "ron") => to_dst(ser_ron(&deser_lexpr(&s)?)?)?,
        #[cfg(all(feature = "lexpr", feature = "bson"))]
        ("sexp", "bson") => write_bin(&ser_bson_to_bin(&deser_lexpr(&s)?)?)?,
        #[cfg(feature = "lexpr")]
        ("sexp", "sexp") => to_dst(ser_lexpr(&deser_lexpr(&s)?)?)?,
        #[cfg(all(feature = "lexpr", feature = "xml"))]
        ("sexp", "xml") => to_dst(ser_xml(&deser_lexpr(&s)?)?)?,
        #[cfg(all(feature = "lexpr", feature = "json5"))]
        ("sexp", "json5") => to_dst(ser_json5(&deser_lexpr(&s)?)?)?,
        */
        #[cfg(feature = "xml")]
        ("xml", "toml") => to_dst(ser_toml(&deser_xml(&s)?)?)?,
        #[cfg(all(feature = "xml", feature = "json"))]
        ("xml", "json") => to_dst(ser_json(&deser_xml(&s)?)?)?,
        #[cfg(all(feature = "xml", feature = "ron"))]
        ("xml", "ron") => to_dst(ser_ron(&deser_xml(&s)?)?)?,
        #[cfg(all(feature = "xml", feature = "yaml"))]
        ("xml", "yaml") => to_dst(ser_yaml(&deser_xml(&s)?)?)?,
        #[cfg(all(feature = "xml", feature = "bson"))]
        ("xml", "bson") => write_bin(&ser_bson_to_bin(&deser_xml(&s)?)?)?,
        #[cfg(all(feature = "xml", feature = "json5"))]
        ("xml", "json5") => to_dst(ser_json5(&deser_xml(&s)?)?)?,
        // #[cfg(feature = "xml")]
        // ("xml", "xml") => to_dst(ser_xml(&deser_xml(&s)?)?)?,
        #[cfg(feature = "json5")]
        ("json5", "toml") => to_dst(ser_toml(&deser_json5(&s)?)?)?,
        #[cfg(feature = "json5")]
        ("json5", "json5") => to_dst(ser_json5(&deser_json5(&s)?)?)?,
        #[cfg(all(feature = "json5", feature = "yaml"))]
        ("json5", "yaml") => to_dst(ser_yaml(&deser_json5(&s)?)?)?,
        #[cfg(all(feature = "json5", feature = "ron"))]
        ("json5", "ron") => to_dst(ser_ron(&deser_json5(&s)?)?)?,
        #[cfg(all(feature = "json5", feature = "bson"))]
        ("json5", "bson") => write_bin(&ser_bson_to_bin(&deser_json5(&s)?)?)?,
        #[cfg(all(feature = "json5", feature = "lexpr"))]
        ("json5", "sexp") => to_dst(ser_lexpr(&deser_json5(&s)?)?)?,
        #[cfg(all(feature = "json5", feature = "xml"))]
        ("json5", "xml") => to_dst(ser_xml(&deser_json5(&s)?)?)?,
        #[cfg(all(feature = "json5", feature = "json"))]
        ("json5", "json") => to_dst(ser_json(&deser_json5(&s)?)?)?,

        (s, d) => {
            log::warn!("{} -> {}", src_fmt, dst_fmt);
            if FORMATS.contains(&d) && s != d {
                log::error!(
                    "{}: {} -> {}",
                    get_conv_text("conv-error")?,
                    s.yellow(),
                    d.magenta()
                );
                log::error!("{}", get_conv_md("not-included")?);
                panic!("{}", get_conv_text("conv-error")?)
            }
            panic!(
                "{}: {} -> {}",
                get_conv_text("unsupported")?,
                src_fmt,
                dst_fmt
            )
        }
    };

    Ok(opt_str)
}
