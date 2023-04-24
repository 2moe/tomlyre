use crate::highlight::syntax::static_syntax_set;
use getset::{Getters, MutGetters};
use glossa::assets::OnceCell;
use std::borrow::Cow;
pub use syntect::{dumps, highlighting::ThemeSet};
use syntect::{highlighting::Theme, parsing::SyntaxSet};
pub mod output;
mod syntax;
pub mod theme;

#[derive(Getters, MutGetters, Debug)]
#[getset(get = "pub with_prefix", get_mut = "pub with_prefix")]
pub struct HighLightRes<'name> {
    name: Cow<'name, str>,
    theme: OnceCell<Theme>,
    theme_set: &'name ThemeSet,
    syntax_set: &'static SyntaxSet,
    background: bool,
}

impl<'name> HighLightRes<'name> {
    pub fn new(name: Cow<'name, str>, theme_set: &'name ThemeSet) -> Self {
        Self {
            name,
            theme_set,
            syntax_set: static_syntax_set(),
            // theme: OnceCell::new(),
            // background: true,
            ..Default::default()
        }
    }

    /// Enable or disable background
    pub fn with_background(self, switch: bool) -> Self {
        Self {
            background: switch,
            ..self
        }
    }
}

impl<'name> Default for HighLightRes<'name> {
    fn default() -> Self {
        Self {
            name: Cow::from(Self::monokai_theme_name()),
            theme: OnceCell::new(),
            syntax_set: static_syntax_set(),
            theme_set: Self::static_theme_set(),
            background: true,
        }
    }
}
