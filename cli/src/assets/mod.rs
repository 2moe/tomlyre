pub(crate) mod localisation;
use glossa::{
    assets::{lang_id_consts, HashMap, OnceCell},
    LangID, MapLoader,
};

use crate::assets::localisation::locale_hashmap;

pub(crate) fn get_l10n_text() -> &'static MapLoader {
    static RES: OnceCell<MapLoader> = OnceCell::new();
    RES.get_or_init(|| MapLoader::new(locale_hashmap()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_log_msg() {
        use glossa::GetText;
        let loader = get_l10n_text();
        let msg = loader.get("log-core", "init-logger");
        dbg!(&msg);
    }
}
