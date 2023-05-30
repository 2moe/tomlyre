use crate::{
    logger::{before_init, init_logger},
    parser::parse_args,
};
mod assets;
mod logger;
mod opt;
mod parser;

const fn get_pkg_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

fn main() -> anyhow::Result<()> {
    let env_name = "TOMLYRE_LOG";
    before_init(get_pkg_name(), env_name);
    init_logger(env_name);
    parse_args()?;
    Ok(())
}

// #[test]
// fn get_parse() {
//     let z = assets::get_l10n_text()
//         .get_or_default("parser_ayu-dark", "sh-completion::about-stderr");
//     println!("{z}")
// }
