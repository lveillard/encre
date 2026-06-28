#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("visibility"),
    values: phf_map! {
        "visible" => "visible",
        "invisible" => "hidden",
    },
});
