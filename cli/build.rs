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
    fs::File,
    io::{self, BufWriter},
    path::PathBuf,
};

fn main() -> io::Result<()> {
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
    // path = PathBuf::from_iter(parent_l10n_dir_arr());
    path = PathBuf::from_iter(
        parent_l10n_dir_arr()
            .into_iter()
            .chain(["cli"]),
    );

    // let mut fmt = HighLightFmt::default();
    let os_str = |s| Cow::from(OsStr::new(s));
    let fname_and_fmt = |s| (os_str(s), HighLightFmt::default());

    let map = HashMap::from_iter([
        fname_and_fmt("opt.toml"),
        fname_and_fmt("conv.toml"),
        fname_and_fmt("args.toml"),
        // fname_and_fmt("parser.yaml"),
    ]);

    // println!("cargo:warning=");
    let mut generator = Generator::new(path).with_version(ver);

    let res = HighLightRes::default();
    *generator.get_highlight_mut() = Some(HighLight::new(res, map));

    generator.run(writer)?;

    Ok(())
}
