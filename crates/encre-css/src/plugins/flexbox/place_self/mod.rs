#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("place-self"),
    values: phf_map! {
        "place-self-auto" => "auto",
        "place-self-start" => "flex-start",
        "place-self-center" => "center",
        "place-self-end" => "flex-end",
        "place-self-stretch" => "stretch",
    },
});
