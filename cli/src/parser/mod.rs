use crate::{
    opt::{args::Cli, theme::get_env_theme_bg, PKG_VERSION},
    parser::{completion::get_shell_completion, sub_cmd::handle_sub_cmds},
};
use anyhow::Result;
use clap::Parser;
use log::{debug, trace};
use log_l10n::level::color::OwoColorize;
use std::{
    borrow::Cow,
    io::{BufWriter, Write},
    process::exit,
};
use tomlyre::table::{set_header, Table, STYLES};
mod completion;
mod set_opts;
mod sub_cmd;

/// The purpose of this constant is to provide sample data for a table, where each tuple represents a row in the table. The first tuple represents a row with version number "10", name "Buster", and create date "2017-06-17", and so on for the remaining tuples.
const TABLE_SAMPLE: [(&str, &str, &str); 5] = [
    ("10", "Buster", "2017-06-17"),
    ("11", "Bullseye", "2019-07-16"),
    ("12", "Bookworm", "2021-08-14"),
    ("13", "Trixie", "2023"),
    ("14", "Forky", "2025"),
];

/// Parses command line arguments using the `Cli::parse()` method, and then perform several actions based on the parsed arguments.
///
/// - gets the syntax highlighting theme style
/// - generates shell completion script
/// - prints version information
/// - gets the table style for displaying output
/// - handles the specific sub-command provided by the parsed arguments.
pub(crate) fn parse_args() -> Result<()> {
    let args = Cli::parse();

    debug!("call the `args.parse_theme_style()` to get the syntax highlighting theme style");
    let theme = {
        let mut t = args.parse_theme_style();
        if *args.get_disable_theme_background() || !get_env_theme_bg() {
            *t.get_background_mut() = false;
        }
        t
    };
    trace!("theme: {:?}", theme);

    debug!("call the `get_shell_completion()` with the parsed arguments and the syntax highlighting theme to generate shell completion scripts");
    trace!("args: {:?}", args);
    get_shell_completion(&args, &theme)?;

    debug!("print the program version information by calling the `print_version` function with the value returned by `args.get_version()`.");
    debug!("get_version: {}", *args.get_version());
    print_version(*args.get_version());

    debug!("call the `get_cli_table_style()` with the parsed arguments to get the table style to be used for displaying output, and stores the result in the `table_style` variable.");
    let table_style = get_table_style_from_cli(&args)?;
    debug!("table_style: {}", table_style);

    debug!("call the `handle_sub_cmd()` with the sub-command provided by the parsed arguments, the syntax highlighting theme, and the table style to handle the specific sub-command.");
    handle_sub_cmds(args.get_sub().as_ref(), &theme, table_style, &args)?;
    Ok(())
}

/// Attempts to get the table style from the Cli struct, which may or may not be present.
///
/// If the style is present but empty, the function iterates through a list of predefined styles, creates a table with sample data for each style, and prints it to stdout.
///
/// If the style is present and not empty, the function checks if the style is valid and returns a Cow containing the style name.
fn get_table_style_from_cli(args: &Cli) -> Result<Cow<str>> {
    Ok(match args.get_table_style().as_ref() {
        Some(s) if s.trim().is_empty() => {
            let mut out = BufWriter::new(std::io::stdout().lock());
            let mut table;

            for style in STYLES {
                writeln!(out, "\n\nstyle: {}\n", style.yellow())?;
                table = Table::new();
                set_header(&mut table, &["Version", "Codename", "Created"], style);

                for (x, y, z) in TABLE_SAMPLE {
                    table.add_row([x, y, z]);
                }
                writeln!(out, "{}", table)?;
            }
            out.flush()?;
            exit(0)
        }
        Some(s) => {
            if !STYLES.contains(&s.as_str()) && s != "md" {
                log::error!("Invalid style name: {}", s.red());
                log::warn!("style list：{:?}", STYLES);
            }
            Cow::from(s)
        }
        _ => Cow::from(""),
    })
}

fn print_version(b: bool) {
    if b {
        println!("{}", PKG_VERSION)
    }
}
