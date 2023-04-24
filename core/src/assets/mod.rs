pub(crate) mod localisation;
use crate::assets::localisation::locale_hashmap;
use glossa::{
    assets::{lang_id_consts, HashMap, OnceCell},
    LangID, MapLoader,
};

pub(crate) fn get_l10n_text() -> &'static MapLoader {
    // Create a new `OnceCell` that can hold a `MapLoader` instance.
    static RES: OnceCell<MapLoader> = OnceCell::new();
    // Retrieve the `MapLoader` instance from the `OnceCell`, or create a new one if it has not been initialised yet.
    RES.get_or_init(|| MapLoader::new(locale_hashmap()))
}
