use crate::parser::parse_args;
mod assets;
mod opt;
mod parser;

const fn get_pkg_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

use log_l10n::logger::{before_init, env_logger};

fn main() -> anyhow::Result<()> {
    let env_name = "TOMLYRE_LOG";
    before_init(get_pkg_name(), env_name);
    env_logger::init(env_name, Some("info"));
    parse_args()?;
    Ok(())
}
