#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("text-underline-offset"),
    values: phf_map! {
        "underline-offset-auto" => "auto",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Number {
    namespace: "underline-offset",
    prop: SingleProp("text-underline-offset"),
    divide_by: 1.0,
})
.template("{}px");

pub(crate) const PLUGIN_3: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "underline-offset",
    prop: SingleProp("text-underline-offset"),
});
