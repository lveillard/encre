#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("list-style-type"),
    values: phf_map! {
        "list-disc" => "disc",
        "list-decimal" => "decimal",
        "list-none" => "none",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "list",
    prop: SingleProp("list-style-type"),
});
