#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_NUMBER: Plugin = Plugin::new(PluginKind::Number {
    namespace: "grid-cols",
    prop: SingleProp("grid-template-columns"),
    divide_by: 1.0,
})
.template("repeat({}, minmax(0, 1fr))");

pub(crate) const PLUGIN_LIST: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-template-columns"),
    values: phf_map! {
        "grid-cols-none" => "none",
    },
});

pub(crate) const PLUGIN_ARBITRARY: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "grid-cols",
    prop: SingleProp("grid-template-columns"),
});
