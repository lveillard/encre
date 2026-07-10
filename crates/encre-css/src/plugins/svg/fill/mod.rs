#![doc = include_str!("README.md")]
#![doc(alias = "svg")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Color {
    namespace: "fill",
    prop: SingleProp("fill"),
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "fill",
    prop: SingleProp("fill"),
});
