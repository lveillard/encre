#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::AnyNumber {
    prop: SingleProp("transition-duration"),
    has_empty: false,
    has_negative: false,
    divide_by: 1.0,
})
.template("{}ms");

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: SingleProp("transition-duration"),
    hints: &[],
    matcher: Time,
});
