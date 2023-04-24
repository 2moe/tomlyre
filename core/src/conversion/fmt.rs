use owo_colors::OwoColorize;

use crate::conversion::{get_conv_dft, serialisation::deser_toml, ConvFmt};
use std::{
    borrow::Cow,
    ffi::OsStr,
    path::{Path, PathBuf},
};

// The array contains some string elements representing different data formats: TOML, YAML (both .yaml and .yml extensions), JSON, RON, BSON, S-expression, XML, and JSON5.
pub(crate) const FORMATS: [&str; 9] = [
    "toml", "yaml", "yml", "json", "ron", "bson", "sexp", "xml", "json5",
];

#[cfg(all(feature = "xml", feature = "json"))]
use crate::conversion::serialisation::deser_xml;

#[cfg(feature = "json")]
use crate::conversion::serialisation::deser_json;

#[cfg(feature = "json5")]
use crate::conversion::serialisation::deser_json5;

#[cfg(feature = "ron")]
use crate::conversion::serialisation::deser_ron;

#[cfg(feature = "yaml")]
use crate::conversion::serialisation::deser_yaml;

impl<'fmt, 'dst, 'src: 'dst> ConvFmt<'fmt, 'dst, 'src> {
    /// This is an implementation of a struct called `ConvFmt` with generic lifetime parameters `'fmt`, `'dst`, and `'src`. The implementation defines a new function, `new`, that constructs and returns an instance of the `ConvFmt` struct.
    ///
    /// Inside the function, a new `ConvFmt` instance is created and initialised with the given args. The source and destination paths are stored as `Cow` objects, which can hold either owned or borrowed data. This allows the `ConvFmt` instance to be constructed with borrowed references to the source and destination paths, which can save memory and avoid unnecessary cloning.
    pub(crate) fn new<P: Into<PathBuf>>(
        src: &'src Path,
        src_fmt: &'fmt str,
        dst_fmt: &'fmt str,
        dst: P,
        save: bool,
    ) -> Self {
        Self {
            src: Cow::from(src),
            src_fmt,
            dst_fmt,
            dst: Cow::from(dst.into()),
            save,
        }
    }
}

// This function takes a string `s` as an argument and returns a static string indicating the detected format, if any.
pub(crate) fn auto_detect_unknown_fmt(s: &str) -> &'static str {
    // Use the `match` expression to check whether the input string can be deserialized using different formats.
    match s.trim_start() {
        // If the input string can be deserialised as TOML format, return "toml".
        s if deser_toml(s).is_ok() => "toml",

        #[cfg(feature = "json")]
        s if deser_json(s).is_ok() => "json",

        #[cfg(feature = "json5")]
        s if deser_json5(s).is_ok() => "json5",

        #[cfg(feature = "yaml")]
        s if deser_yaml(s).is_ok() => "yaml",

        #[cfg(feature = "ron")]
        s if deser_ron(s).is_ok() => "ron",

        // If the feature flags for both XML and JSON are enabled and the input string can be deserialised as XML format, return "xml".
        #[cfg(all(feature = "xml", feature = "json"))]
        s if deser_xml(s).is_ok() => "xml",

        // If the input string cannot be deserialised as any of the above formats, log an error message and panic with a custom message.
        _ => {
            log::error!("{}", get_conv_dft("unknown-fmt"));
            log::error!("{}: {:?}", get_conv_dft("currently-supported"), FORMATS);
            panic!("{}", get_conv_dft("auto-detection-failed"))
        }
    }
}

// Maps "yml" file extension to its equivalent format "yaml".
pub(crate) fn yml_is_yaml(s: &str) -> &str {
    if s == "yml" {
        "yaml"
    } else {
        s
    }
}

// Maps "json5" file extension to its equivalent format "json".
// pub(crate) fn json5_as_json(s: &str) -> &str {
//     if s == "json5" {
//         "json"
//     } else {
//         s
//     }
// }

// Returns the format of a given source file based on its file extension.
pub(crate) fn get_src_format(src: &Path) -> &str {
    match src
        .extension()
        .and_then(|o| o.to_str())
    {
        // If the file extension matches any of the supported formats, return it as is.
        Some(s) if FORMATS.contains(&s) => yml_is_yaml(s),
        // If the file extension does not match any of the supported formats, log a warning message and return an empty string.
        _ => {
            log::warn!(
                "Unknown format, file name: {:?}",
                src.file_name()
                    .unwrap_or_else(|| OsStr::new("UNKNOWN"))
            );
            log::warn!("Attempting to automatically detect the file format. If detection fails, please specify the format manually.");
            ""
        }
    }
}

pub(crate) fn get_diff_dst_format(src_fmt: &str) -> &str {
    match src_fmt {
        "json5" => "json",
        "toml" => "yaml",
        "yaml" => "json",
        _ => "toml",
    }
}

// Returns the format to which a file should be converted based on its content.
pub(crate) fn get_different_file_format(s: &str) -> &str {
    get_diff_dst_format(check_supported_formats(s))
}

// Checks whether a given file extension is supported and returns it if it is.
pub(super) fn check_supported_formats(ext: &str) -> &str {
    FORMATS
        .iter()
        .find(|&&x| x == ext)
        .copied()
        .unwrap_or_else(|| {
            log::warn!("{}: {}", get_conv_dft("unknown-fmt"), ext.purple());
            ""
        })
}

pub(super) fn trim_unknown_fmt(ext: &str) -> Option<&str> {
    let trimmed = match ext.len() {
        0..=2 => ext,
        3..=4 => &ext[..1],
        _ => &ext[..2],
    };
    FORMATS
        .iter()
        .find(|&&x| x.starts_with(trimmed))
        .copied()
}
