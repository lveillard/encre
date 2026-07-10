#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-auto-rows"),
    values: phf_map! {
        "auto-rows-auto" => "auto",
        "auto-rows-min" => "min-content",
        "auto-rows-max" => "max-content",
        "auto-rows-fr" => "minmax(0, 1fr)"
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "auto-rows",
    prop: SingleProp("grid-auto-rows"),
});
