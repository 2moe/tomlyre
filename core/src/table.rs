pub use comfy_table::Table;

use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::{
        ASCII_BORDERS_ONLY_CONDENSED, ASCII_FULL, ASCII_HORIZONTAL_ONLY,
        ASCII_MARKDOWN, NOTHING, UTF8_BORDERS_ONLY, UTF8_FULL, UTF8_FULL_CONDENSED,
        UTF8_NO_BORDERS,
    },
    Cell, ContentArrangement,
};

#[cfg(not(target_family = "wasm"))]
use comfy_table::{
    Attribute, // Styling attributes for cells
    Color,     // Text color of cells
};

use std::fmt::Display;
// Constant &str for using UTF-8 border characters
const UTF8_THIN: &str = "││──├─┼┤┆╌┼├┤┬┴┌┐└┘";

const UTF8_NORMAL: &str = "││──├─┼┤│─┼├┤┬┴┌┐└┘";

pub const STYLES: [&str; 15] = [
    "default",
    "nothing",
    "ascii",
    "ascii-borders",
    "right-u8-fat",
    "right-u8",
    "right-u8-thin",
    "u8",
    "u8-no-dividers",
    "u8-borders",
    "u8-no-borders",
    "horizontal",
    "round-u8",
    "round-u8-fat",
    "markdown",
];

pub fn set_header<H>(table: &mut Table, header: &[H], style: &str)
where
    H: Display,
{
    let right = UTF8_SOLID_INNER_BORDERS;
    let round = UTF8_ROUND_CORNERS;
    let thin = UTF8_THIN;

    match style {
        "md" | "markdown" => table.load_preset(ASCII_MARKDOWN),
        "horizontal" => table.load_preset(ASCII_HORIZONTAL_ONLY),
        "ascii" => table.load_preset(ASCII_FULL),
        "ascii-borders" => table.load_preset(ASCII_BORDERS_ONLY_CONDENSED),
        "u8" => table.load_preset(UTF8_FULL),
        "u8-no-dividers" => table.load_preset(UTF8_FULL_CONDENSED),
        "u8-borders" => table.load_preset(UTF8_BORDERS_ONLY),
        "u8-no-borders" => table.load_preset(UTF8_NO_BORDERS),
        "nothing" => table.load_preset(NOTHING),

        "right-u8-fat" => table
            .load_preset(UTF8_FULL)
            .apply_modifier(right),
        "right-u8" => table.load_preset(UTF8_NORMAL),
        "right-u8-thin" => table.load_preset(thin),

        "round-u8" => table
            .load_preset(UTF8_NORMAL)
            .apply_modifier(round),
        "round-u8-fat" => table
            .load_preset(UTF8_FULL)
            .apply_modifier(round),
        // "round-u8-thin"
        _ => table
            .load_preset(thin)
            .apply_modifier(round),
    }
    .set_content_arrangement(ContentArrangement::Dynamic)
    .set_header(header.iter().map(|x| {
        #[cfg(not(target_family = "wasm"))]
        {
            // `map` transforms each element of `header` into a `Cell`
            Cell::new(x) // Create a new cell with the displayable item in `x`
                .add_attribute(Attribute::Bold) // Add bold styling
                .fg(Color::Cyan) // Set the text color to cyan
        }
        #[cfg(target_family = "wasm")]
        {
            Cell::new(x)
        }
    }));
}
