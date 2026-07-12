#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("transition-timing-function"),
    values: phf_map! {
        "ease-linear" => "linear",
        "ease-in" => "cubic-bezier(0.4, 0, 1, 1)",
        "ease-out" => "cubic-bezier(0, 0, 0.2, 1)",
        "ease-in-out" => "cubic-bezier(0.4, 0, 0.2, 1)",
    },
});

pub(crate) const PLUGIN_ARBITRARY: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "ease",
    prop: SingleProp("transition-timing-function"),
});
