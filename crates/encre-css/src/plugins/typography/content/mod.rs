#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::SamePropValues {
    prop: SingleProp("--en-content"),
    values: &["none"],
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("--en-content"),
    hints: &[],
    matcher: All,
});
