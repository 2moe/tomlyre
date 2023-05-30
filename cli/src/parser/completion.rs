use crate::{assets::get_l10n_text, opt::args::Cli};
use clap::CommandFactory;
use clap_complete::{generate, shells, Generator};
use glossa::GetText;
use hlight::{gen_syntax_highlight, HighLightRes};
use log::info;
use std::{
    env,
    io::{self, BufWriter, Write},
};
// use tomlyre::highlight::{output::gen_syntax_highlight, HighLightRes};

const BIN_NAME: &str = env!("CARGO_BIN_NAME");

/// Generates shell completion scripts using clap_complete.
/// It takes a generic type parameter G which specifies the type of shell to generate the script for, and returns a byte vector containing the generated script.
fn gen_completion<G: Generator>(g: G) -> Vec<u8> {
    let mut cmd = Cli::command();
    let mut s = Vec::with_capacity(1024 * 13);
    generate(g, &mut cmd, BIN_NAME, &mut s);
    s
}

/// Generates a shell completion script based on the specified shell type in args, and prints it out.
///
/// Note: It will output the raw text to stdout first, and then the highlighted text to stderr.
///
/// It uses a match expression to determine which type of shell to generate the script for, and calls the gen_completion function with the appropriate generator type.
/// It then uses the `print_syntax_highlight()` function to print the highlighted script to standard error, and flushes the output.
pub(crate) fn get_shell_completion(
    args: &Cli,
    theme: &HighLightRes,
) -> anyhow::Result<()> {
    let Some(sh) = args.get_shell_completion() else { return Ok(()) };

    fn gen<G: Generator>(g: G) -> Vec<u8> {
        gen_completion(g)
    }

    let sh_name = sh.to_ascii_lowercase();

    let v = {
        use shells::*;
        match sh_name.as_str() {
            "bash" => gen(Bash),
            "zsh" => gen(Zsh),
            "pwsh" | "powershell" => gen(PowerShell),
            "fish" => gen(Fish),
            "elvish" => gen(Elvish),
            _ => {
                info!("system shell: {:?}", env::var("SHELL"));
                panic!("Unsupported shell: {}", sh_name)
            }
        }
    };
    let s = String::from_utf8_lossy(&v);

    println!("{s}");
    let mut out = BufWriter::new(io::stderr().lock());
    info!(
        "{}",
        get_l10n_text().get_or_default("parser", "sh-completion::about-stderr")
    );

    let fmt = match sh_name.as_str() {
        s @ "fish" => s,
        "pwsh" | "powershell" => "ps1",
        "elvish" => "elv",
        _ => "zsh",
    };

    gen_syntax_highlight(fmt, &s, Some(theme), Some(&mut out))?;
    out.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn get_zsh_comp() -> Result<()> {
        let v = gen_completion(shells::Zsh);
        let s = String::from_utf8_lossy(&v);
        let res = HighLightRes::default().with_background(false);

        // let syntax = res.get_syntax_set().syntaxes();
        // for i in syntax.iter().map(|x| &x.name) {
        //     println!("{i}")
        // }
        gen_syntax_highlight("zsh", &s, Some(&res), None)?;

        Ok(())
    }

    #[test]
    fn zsh_high_light() -> io::Result<()> {
        let s = r#"
        #compdef tomlyre

        autoload -U is-at-least
        
        _tomlyre() {
            typeset -A opt_args
            typeset -a _arguments_options
            local ret=1
        
            if is-at-least 5.2; then
                _arguments_options=(-s -S -C)
            else
                _arguments_options=(-s -C)
            fi
        
            local context curcontext="$curcontext" state line
            _arguments "${_arguments_options[@]}" \
        "#;

        let res = HighLightRes::default();
        gen_syntax_highlight("pwsh", s, Some(&res), None)
    }
}
