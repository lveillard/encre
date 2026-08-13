#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("text-underline-offset"),
    values: phf_map! {
        "underline-offset-auto" => "auto",
    },
});

pub(crate) const PLUGIN_NUMBER: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "underline-offset",
    prop: SingleProp("text-underline-offset"),
})
.template("{}px");

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "underline-offset",
    prop: SingleProp("text-underline-offset"),
});
