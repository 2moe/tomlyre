use crate::highlight::HighLightRes;
use glossa::assets::OnceCell;
use syntect::{
    dumps,
    highlighting::{Theme, ThemeSet},
};

const THEME_SET: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/syntect/theme-set.packdump"
));

fn default_theme_set() -> ThemeSet {
    dumps::from_uncompressed_data(THEME_SET).unwrap()
}

impl<'name> HighLightRes<'name> {
    pub fn set_theme_once(&self) -> &Theme {
        self.get_theme()
            .get_or_init(|| {
                let name = self.get_name().as_ref();
                let set = self.get_theme_set();
                set.themes[name].to_owned()
            })
    }

    pub fn static_theme_set() -> &'static ThemeSet {
        static S: OnceCell<ThemeSet> = OnceCell::new();
        S.get_or_init(default_theme_set)
    }

    pub const fn monokai_theme_name() -> &'static str {
        "Monokai Extended"
    }

    pub const fn ayu_dark_theme_name() -> &'static str {
        "ayu-dark"
    }
}
