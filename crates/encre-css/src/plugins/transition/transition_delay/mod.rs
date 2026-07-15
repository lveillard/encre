#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "delay",
    prop: SingleProp("transition-delay"),
    divide_by: 1.0,
})
.template("{}ms");

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "delay",
    prop: SingleProp("transition-delay"),
});
