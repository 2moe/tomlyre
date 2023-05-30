const EARLY_RT: bool = true;
// const EARLY_RT: bool = false;

use glossa_codegen::{
    consts::*,
    highlight::{HighLight, HighLightFmt, HighLightRes},
    prelude::*,
};
use std::{borrow::Cow, collections::HashMap, ffi::OsStr, io, path::PathBuf};

fn main() -> io::Result<()> {
    let ver = get_pkg_version!();

    let rs_path = PathBuf::from_iter(default_l10n_rs_file_arr());

    if is_same_version(&rs_path, Some(ver))? && EARLY_RT {
        return Ok(());
    }
    append_to_l10n_mod(&rs_path)?;

    // let file = BufWriter::new(File::create(&rs_path)?);
    let tmp = rs_path.with_extension("tmp");
    let mut writer = MapWriter::new(&tmp, &rs_path);

    *writer.get_visibility_mut() = "pub(super)";

    // The directory containing the localisation data
    let l10n_path = PathBuf::from_iter(
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
    let mut generator = Generator::new(l10n_path).with_version(ver);

    let res = HighLightRes::default();
    *generator.get_highlight_mut() = Some(HighLight::new(res, map));

    generator.run(writer)
}
