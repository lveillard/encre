#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::SamePropValues {
    prop: SingleProp("border-style"),
    values: &[
        "solid", "dashed", "dotted", "double", "groove", "ridge", "inset", "outset", "hidden",
        "none",
    ],
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("border-style"),
    hints: &[],
    matcher: SpaceSeparated(&LineStyle),
});
