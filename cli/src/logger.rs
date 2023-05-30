use env_logger::Env;
use glossa::GetText;
use owo_colors::{OwoColorize, Style};
use std::{env, io::Write};
use time::OffsetDateTime;

use crate::assets::get_l10n_text;

pub(crate) fn get_log_core_l10n(key: &str) -> &str {
    get_l10n_text()
        .get("log-core", key)
        .unwrap_or(key)
}

/// Matches a log level with a corresponding string and style for formatting purposes.
///
/// The function first creates a new `Style` object using the `new()` method.
///
/// It then matches the input log level against several variants of the `log::Level` enum using a `match` expression.
///
/// For each variant, it selects the corresponding index in the input array and returns a tuple containing that string and a modified `Style` object with specific styling properties applied based on the log level.
///
/// If the input log level does not match any of the expected variants, the function returns an empty string slice and an unmodified `Style` object.
fn match_log_level<'a>(lv: &log::Level, arr: [&'a str; 5]) -> (&'a str, Style) {
    use log::Level::*;
    let style = Style::new();

    match lv {
        Error => (arr[0], style.bold().bright_red()),
        Warn => (
            arr[1],
            style
                .bold()
                .yellow()
                .underline(),
        ),
        Info => (arr[2], style.green()),
        Debug => (arr[3], style.blue()),
        Trace => (arr[4], style.cyan()),
        #[allow(unreachable_patterns)]
        _ => ("", style),
    }
}

/// Gets the current time with an offset from UTC.
///
/// It uses the `now_local` method on the `OffsetDateTime` struct to get the current local time, and if that fails (for example, if the system clock is not set correctly), it falls back to getting the current UTC time using the `now_utc` method.
pub(crate) fn get_offset_time() -> time::OffsetDateTime {
    OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc())
}

pub(crate) fn before_init(pkg: &str, env: &str) {
    if let Ok("debug") | Ok("trace") = env::var(env).as_deref() {
        eprintln!(
            "{} [{}] {}: {}",
            get_offset_time(),
            get_l10n_text()
                .get_or_default("log-core", "debug")
                .blue(),
            pkg,
            get_l10n_text().get_or_default("log-core", "init-logger")
        )
    }
}

/// Initialises a logger using the `env_logger` crate. It sets up the log levels and formatting for log messages.
///
/// First creates an array of string slices representing the log levels in decreasing order of severity, and then maps each log level to its localised version using the `get_log_core_l10n()`. The resulting array is stored in `lv_l10n`.
///
/// Next, the function creates a new `env_logger::Builder` from the default environment variables and sets the default filter level to "info" if none is specified.
/// It then sets up a custom log format using a closure passed to the `format` method.
///
/// The closure uses the `get_offset_time` function to get the current time with an offset, and formats it as a timestamp with milliseconds and an optional offset indicator ("Z" for UTC).
/// It then calls the `match_log_level` function to get the localised log level string and corresponding `Style`.
/// Finally, it formats the log message using the provided `fmt` parameter and the log record's `module_path`, `line`, and `args`.
/// The log level is styled using the `style` method.
///
/// The resulting logger configuration is initialised using the `init` method on the `env_logger::Builder`.
pub(crate) fn init_logger(env_name: &str) {
    let arr = ["error", "warn", "info", "debug", "trace"];
    let mut lv_l10n = arr;

    for (i, value) in arr
        .into_iter()
        .map(get_log_core_l10n)
        .enumerate()
    {
        unsafe { *lv_l10n.get_unchecked_mut(i) = value }
    }

    env_logger::Builder::from_env(Env::new().filter_or(env_name, "info"))
        .format(move |fmt, r| {
            let now = get_offset_time();
            let offset_str = if now.offset().is_utc() { "Z" } else { "" };
            let (lv, sty) = match_log_level(&r.level(), lv_l10n);

            writeln!(
                fmt,
                "{:02}:{:02}:{:02}.{:03}{} [{}] {}:{}  {}",
                now.hour(),
                now.minute(),
                now.second(),
                now.millisecond(),
                offset_str,
                lv.style(sty),
                r.module_path().unwrap_or(""),
                r.line().unwrap_or(0).blue(),
                r.args()
            )
        })
        .init();
}
