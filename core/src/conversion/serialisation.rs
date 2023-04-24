#[cfg(feature = "bson")]
use std::{fs::File, path::Path};

use anyhow::Result;
use serde::Serialize;

pub(crate) fn deser_toml(s: &str) -> Result<toml::Value> {
    Ok(toml::from_str(s)?)
}

#[cfg(feature = "yaml")]
pub(crate) fn deser_toml_to_yaml(s: &str) -> Result<serde_yaml::Value> {
    Ok(toml::from_str(s)?)
}

pub(crate) fn ser_toml<S: Serialize>(s: &S) -> Result<String> {
    Ok(toml::to_string_pretty(s)?)
}
#[cfg(feature = "yaml")]
pub(crate) fn deser_yaml(s: &str) -> Result<serde_yaml::Value> {
    Ok(serde_yaml::from_str(s)?)
}

#[cfg(feature = "yaml")]
pub(crate) fn ser_yaml<S: Serialize>(s: &S) -> Result<String> {
    Ok(serde_yaml::to_string(s)?)
}

#[cfg(feature = "json")]
pub(crate) fn deser_json(s: &str) -> Result<serde_json::Value> {
    Ok(serde_json::from_str(s)?)
}
#[cfg(feature = "json")]
pub(crate) fn ser_json<S: Serialize>(s: &S) -> Result<String> {
    Ok(serde_json::to_string_pretty(&s)?)
}

#[cfg(all(feature = "json5", feature = "json"))]
pub(crate) fn deser_json5(s: &str) -> Result<serde_json::Value> {
    Ok(json5::from_str(s)?)
}
#[cfg(feature = "json5")]
pub(crate) fn ser_json5<S: Serialize>(s: &S) -> Result<String> {
    Ok(json5::to_string(s)?)
}

#[cfg(feature = "ron")]
pub(crate) fn deser_ron(s: &str) -> Result<ron::Value> {
    Ok(ron::from_str(s)?)
}

#[cfg(feature = "ron")]
pub(crate) fn ser_ron<S: Serialize>(s: &S) -> Result<String> {
    Ok(ron::ser::to_string_pretty(
        s,
        ron::ser::PrettyConfig::default()
            .depth_limit(64)
            .enumerate_arrays(true),
    )?)
}

#[cfg(feature = "bson")]
pub(crate) fn deser_bson(file: &Path) -> Result<bson::Bson> {
    use crate::conversion::is_src_stdin;
    use log::info;
    use std::io::{self, Read};

    if is_src_stdin(file) {
        info!("Getting bson data from stdin");
        let mut buf = Vec::with_capacity(1024);
        io::stdin().read_to_end(&mut buf)?;
        return Ok(bson::from_slice_utf8_lossy(&buf)?);
    }
    Ok(bson::from_reader_utf8_lossy(File::open(file)?)?)
}

#[cfg(feature = "bson")]
pub(crate) fn ser_bson_to_bin<S: Serialize>(s: &S) -> Result<Vec<u8>> {
    Ok(bson::to_vec(s)?)
}
/*
#[cfg(all(feature = "lexpr", feature = "ron"))]
pub(crate) fn deser_lexpr(s: &str) -> Result<ron::Value> {
    Ok(serde_lexpr::from_str(s)?)
}
*/
#[cfg(feature = "lexpr")]
pub(crate) fn ser_lexpr<S: Serialize>(s: &S) -> Result<String> {
    Ok(serde_lexpr::to_string_custom(
        &s,
        serde_lexpr::print::Options::elisp(),
    )?)
}

#[cfg(all(feature = "xml", feature = "json"))]
pub(crate) fn deser_xml(s: &str) -> Result<serde_json::Value> {
    // Ok(quick_xml::de::from_str(s)?)
    Ok(quickxml_to_serde::xml_str_to_json(
        s,
        &quickxml_to_serde::Config::default(),
    )?)
}

#[cfg(feature = "xml")]
pub(crate) fn ser_xml<S: Serialize>(s: &S) -> Result<String> {
    quick_xml::se::to_string(&s).or_else(|e| {
        log::error!("{e}");
        log::warn!(
            "A root tag, named Root, will be automatically added to this file."
        );
        Ok(quick_xml::se::to_string_with_root("Root", &s)?)
    })
}
