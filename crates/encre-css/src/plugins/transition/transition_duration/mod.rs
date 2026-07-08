#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Number {
    prefix: "duration",
    prop: SingleProp("transition-duration"),
    divide_by: 1.0,
})
.template("{}ms");

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "duration",
    prop: SingleProp("transition-duration"),
});
