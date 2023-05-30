const EARLY_RT: bool = true;
// const EARLY_RT: bool = false;

use glossa_codegen::{
    consts::*,
    highlight::{HighLight, HighLightFmt, HighLightRes},
    prelude::*,
};
use std::{borrow::Cow, collections::HashMap, ffi::OsStr, io, path::PathBuf};

fn main() -> io::Result<()> {
    gen_10n_text()?;
    Ok(())
}

fn gen_10n_text() -> io::Result<()> {
    let ver = get_pkg_version!();

    // path: "src"/"assets"/"localisation.rs"
    let rs_path = PathBuf::from_iter(default_l10n_rs_file_arr());

    if is_same_version(&rs_path, Some(ver))? && EARLY_RT {
        return Ok(());
    }

    append_to_l10n_mod(&rs_path)?;

    let tmp = get_shmem_path(&rs_path)?;
    let mut writer = MapWriter::new(&tmp, &rs_path);
    *writer.get_visibility_mut() = "pub(super)";

    // path: ../assets/l10n/core
    let l10n_path = PathBuf::from_iter(
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

    let mut generator = Generator::new(l10n_path).with_version(ver);

    let res = HighLightRes::default();
    *generator.get_highlight_mut() = Some(HighLight::new(res, map));

    generator.run(writer)
}
