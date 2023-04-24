use crate::{
    assets::get_l10n_text,
    logger::{get_offset_time, init_logger},
    parser::parse_args,
};
use glossa::GetText;
use owo_colors::OwoColorize;
use std::env;
mod assets;
mod logger;
mod opt;
mod parser;

fn main() -> anyhow::Result<()> {
    if let Ok("debug") | Ok("trace") = env::var("RUST_LOG").as_deref() {
        eprintln!(
            "{} [{}] tomlyre: {}",
            get_offset_time(),
            get_l10n_text()
                .get_or_default("log-core", "debug")
                .blue(),
            get_l10n_text().get_or_default("log-core", "init-logger")
        )
    }
    init_logger();
    parse_args()?;
    Ok(())
}

#[test]
fn get_parse() {
    let z = get_l10n_text()
        .get_or_default("parser_ayu-dark", "sh-completion::about-stderr");
    println!("{z}")
}
