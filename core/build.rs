const EARLY_RT: bool = true;
// const EARLY_RT: bool = false;

use glossa_codegen::{
    consts::*,
    highlight::{HighLight, HighLightFmt, HighLightRes},
    prelude::*,
};

use std::{
    borrow::Cow,
    collections::HashMap,
    ffi::OsStr,
    fs::{self, File},
    io::{self, BufWriter},
    path::PathBuf,
};
use syntect::{
    dumps::dump_to_uncompressed_file, highlighting::ThemeSet,
    parsing::SyntaxSetBuilder,
};

fn get_syntect_dir() -> PathBuf {
    PathBuf::from_iter(["assets", "syntect"])
}

fn get_parent_assets_dir() -> PathBuf {
    PathBuf::from_iter(["..", "assets"])
}

fn dump_syntax_set() {
    let mut ss = SyntaxSetBuilder::default();
    ss.add_from_folder(get_parent_assets_dir().join("syntax"), true)
        .expect("Failed to add syntax dir");

    let set = ss.build();
    dump_to_uncompressed_file(&set, get_syntect_dir().join("syntax-set.packdump"))
        .expect("Failed to dump syntax");
}

fn dump_theme_set() -> io::Result<()> {
    let ts = ThemeSet::load_from_folder(get_parent_assets_dir().join("theme"))
        .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e.to_string()))?;

    dump_to_uncompressed_file(&ts, get_syntect_dir().join("theme-set.packdump"))
        .expect("Failed to dump theme");

    Ok(())
}

fn main() -> io::Result<()> {
    gen_10n_text()?;
    // const PKG_VER: &str = env!("CARGO_PKG_VERSION");
    const PKG_VER: &str = "2023-04-13_14";
    let ver_txt = get_syntect_dir().join("version.txt");

    if ver_txt.exists() && fs::read_to_string(&ver_txt)?.trim() == PKG_VER {
        return Ok(());
    }

    fs::write(ver_txt, PKG_VER)?;
    dump_theme_set()?;
    dump_syntax_set();
    Ok(())
}

fn gen_10n_text() -> io::Result<()> {
    let ver = get_pkg_version!();

    // Create a new `PathBuf` from the result of calling `get_l10n_rs_file_arr()`
    let mut path = PathBuf::from_iter(default_l10n_rs_file_arr());

    if is_same_version(&path, Some(ver))? && EARLY_RT {
        return Ok(());
    }

    append_to_l10n_mod(&path)?;

    let file = BufWriter::new(File::create(&path)?);

    let mut writer = MapWriter::new(file);
    *writer.get_visibility_mut() = "pub(super)";

    // Update the `PathBuf` to point to the directory containing the localisation data
    path = PathBuf::from_iter(
        parent_l10n_dir_arr()
            .into_iter()
            .chain(["core"]),
    );

    let os_str = |s| Cow::from(OsStr::new(s));
    let fname_and_fmt = |s| (os_str(s), HighLightFmt::default());

    let map = HashMap::from_iter([
        fname_and_fmt("conversion.toml"),
        fname_and_fmt("set.toml"),
        // fname_and_fmt("parser.yaml"),
    ]);

    let mut generator = Generator::new(path).with_version(ver);

    let res = HighLightRes::default();
    *generator.get_highlight_mut() = Some(HighLight::new(res, map));

    generator.run(writer)?;

    Ok(())
}
