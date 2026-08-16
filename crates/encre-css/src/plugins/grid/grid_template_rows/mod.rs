#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_NUMBER: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "grid-rows",
    prop: SingleProp("grid-template-rows"),
})
.template("repeat({}, minmax(0, 1fr))");

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-template-rows"),
    values: map! {
        "grid-rows-none" => "none",
    },
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "grid-rows",
    prop: SingleProp("grid-template-rows"),
});
