#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Number {
    namespace: "grid-rows",
    prop: SingleProp("grid-template-rows"),
    divide_by: 1.0,
})
.template("repeat({}, minmax(0, 1fr))");

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-template-rows"),
    values: phf_map! {
        "grid-rows-none" => "none",
    },
});

pub(crate) const PLUGIN_3: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "grid-rows",
    prop: SingleProp("grid-template-rows"),
});
