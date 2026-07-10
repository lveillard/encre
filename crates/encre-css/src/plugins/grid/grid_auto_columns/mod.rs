#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-auto-columns"),
    values: phf_map! {
        "auto-cols-auto" => "auto",
        "auto-cols-min" => "min-content",
        "auto-cols-max" => "max-content",
        "auto-cols-fr" => "minmax(0, 1fr)"
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "auto-cols",
    prop: SingleProp("grid-auto-columns"),
});
