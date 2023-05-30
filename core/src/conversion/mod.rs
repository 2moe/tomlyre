use crate::{
    assets::get_l10n_text,
    conversion::{
        convert::convert_data_format,
        fmt::{
            auto_detect_unknown_fmt, check_supported_formats,
            get_different_file_format, get_src_format, trim_unknown_fmt,
            yml_is_yaml,
        },
    },
};
// json5_as_json,
use anyhow::Result;
use glossa::{assets::OnceCell, GetText};
use hlight::HighLightRes;
use std::{
    borrow::Cow,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

pub(crate) mod convert;
pub(crate) mod fmt;
pub(crate) mod serialisation;

/// Represents the source and destination format of a file to be converted.
///
/// - `src`: a reference to a `Path` object representing the source file path.
/// - `src_fmt`: a string slice representing the format of the source file.
/// - `dst_fmt`: a string slice representing the desired output format.
/// - `dst`: a generic type `P` that can be converted into a `PathBuf` object representing the destination file path.
/// - `save`: a boolean value indicating whether the converted data should be saved in the destination file.
#[derive(Debug, Default)]
pub(crate) struct ConvFmt<'fmt, 'src, 'dst> {
    src: Cow<'src, Path>,
    // src_str: Option<Cow<'src, str>>,
    src_fmt: &'fmt str,
    dst_fmt: &'fmt str,
    dst: Cow<'dst, Path>,
    save: bool,
}

/// Checks whether the given source file is from stdin or not.
pub(crate) fn is_src_stdin(src: &Path) -> bool {
    matches!(src.as_os_str().to_str(), Some("-"))
}

/// Gets the static data from stdin.
pub(crate) fn get_static_stdin_data() -> &'static str {
    static STDIN: OnceCell<String> = OnceCell::new();
    STDIN.get_or_init(|| {
        let mut s = String::with_capacity(64);
        log::warn!("Getting data from stdin");
        io::stdin()
            .read_to_string(&mut s)
            .expect("Failed to get data from stdin");
        s
    })
}

/// Performs the file conversion task.
pub fn task(
    src: &Path,
    dst: Option<&Vec<PathBuf>>,
    save: bool,
    src_format: Option<&String>,
    high_light_resource: Option<&HighLightRes>,
) -> Result<()> {
    // Get the source format based on the given parameters.
    let src_fmt = match src_format {
        // If the source format is explicitly specified, use it.
        Some(s) => s,
        // If the source file is stdin, automatically detect the format using its data.
        _ if is_src_stdin(src) => auto_detect_unknown_fmt(get_static_stdin_data()),
        // Otherwise, try to get the format from the file extension of the source file. If that fails, attempt to automatically detect the format using the content of the file.
        _ => match get_src_format(src) {
            s if s.is_empty() => auto_detect_unknown_fmt(&fs::read_to_string(src)?),
            s => s,
        },
    };

    // If no destination options are provided, convert the source file to a single output file with a format that is different from the source format.
    match dst {
        None => {
            let dst_fmt = get_different_file_format(src_fmt);
            convert_data_format(
                ConvFmt::new(
                    src,
                    src_fmt,
                    dst_fmt,
                    src.with_extension(dst_fmt),
                    save,
                ),
                false,
                high_light_resource,
            )?;
        }
        // If destination options are provided, convert the source file to multiple output files with different formats.
        Some(dst_path_list) => {
            write_to_dst(dst_path_list, src, save, src_fmt, high_light_resource)?;
        }
    };
    Ok(())
}

/// Writes the converted data to the destination file(s).
pub(crate) fn write_to_dst(
    dst_path_list: &[PathBuf],
    src: &Path,
    save: bool,
    src_fmt: &str,
    high_light_resource: Option<&HighLightRes>,
) -> Result<()> {
    for (org_dst, dst_fname) in dst_path_list
        .iter()
        .map(|x| (x, x.file_name()))
        .filter_map(|(x, f)| {
            f.map(|new_f| {
                (
                    x,
                    new_f
                        .to_string_lossy()
                        .to_ascii_lowercase(),
                )
            })
        })
    {
        let dst_fmt =
            get_dst_fmt(&dst_fname, true).unwrap_or_else(|| 
                // 
                // json5_as_json(
                    src_fmt
                // )
                );

        let dst = get_dst(org_dst, dst_fmt, src);

        // If the destination file has the same path as the source file and `save` is true, log a warning message and skip the file.
        if src == dst && save {
            log::warn!(
                "Please use a different file extension. All supported extensions are {:?}",
                fmt::FORMATS
            );
            log::error!(
                "Input and output files cannot be the same, skipping: {}",
                dst.display()
            );
            continue;
        }

        // Convert the source data to the destination format and write it to the destination file.
        convert_data_format(
            ConvFmt::new(src, src_fmt, dst_fmt, dst, save),
            false,
            high_light_resource,
        )?;
    }
    Ok(())
}

/// Used to get the path to the dst file.
///
/// Here's what `get_dst` does:
///
/// 1. If `org` has no extension and only one component (i.e., it's just a filename with no path), then:
///     - If the filename equals `dst_fmt`, return `src` with the extension `dst_fmt`.
///     - If the filename equals "yml", return `src` with the extension "yml".
///     - If `org` is a directory, return `org` with the filename from `src` and the extension `dst_fmt`.
///     - Otherwise, return `org` with the extension `dst_fmt`.
/// 2. Otherwise (if `org` has an extension), return `org`.
pub(crate) fn get_dst(org: &Path, dst_fmt: &str, src: &Path) -> PathBuf {
    let new_dir_path =
        || Path::new(src.file_name().unwrap()).with_extension(dst_fmt);

    match org.extension() {
        None if org.components().count() == 1 => match org.to_str() {
            Some(o) if o == dst_fmt => src.with_extension(dst_fmt),
            Some(o) if o == "yml" => src.with_extension("yml"),
            _ if org.is_dir() => org.join(new_dir_path()),
            _ => org.with_extension(dst_fmt),
        },
        _ if org.is_dir() => org.join(new_dir_path()),
        _ => org.into(),
    }
}

/// Used to get the format of the dst file.
///
/// Takes a single arg: `fname`, which is a string representing a filename. It returns an optional string.
///
/// Here's what `get_dst_fmt` does:
///
/// 1. Split `fname` using `rsplit('.')`, and get the first element(i.e. its extension). The first element of rsplit is equivalent to the last element of split.
///     - If the file extension does not exist, then call to `Path::extension()` will be None. The use of rsplit ensures that the file ext is present. If the src file name is `yml`, it does not have `.`, but rsplit will still fetch `yml`. If it is empty for some reason, it will be filtered out in the next `filter`.
/// 2. Check if the extension is supported using `check_supported_formats`.
/// 3. Filter out any empty extensions(formats).
/// 4. If the extension is "yml", return "yaml" instead.
/// 5. If `auto_guess` is true, then `jsop` -> `jso` -> `json`, `js` -> `json`.
/// 6. Return the extension(format) as an optional string.
pub(crate) fn get_dst_fmt(fname: &str, auto_guess: bool) -> Option<&str> {
    let ext = fname.rsplit('.').next();

    let sup = ext.map(check_supported_formats);

    if sup
        .filter(|s| !s.trim().is_empty())
        .is_some()
    {
        return sup.map(yml_is_yaml);
    }

    if !auto_guess {
        return None;
    }

    trim_unknown_fmt(ext.unwrap_or(fname)).map(yml_is_yaml)
}

fn get_conv_text(k: &str) -> glossa::Result<&str> {
    get_l10n_text().get("conversion", k)
}

pub(crate) fn get_conv_dft(k: &str) -> Cow<str> {
    get_l10n_text().get_or_default("conversion", k)
}

fn get_conv_md(k: &str) -> glossa::Result<&str> {
    get_l10n_text().get("conversion_md", k)
}
