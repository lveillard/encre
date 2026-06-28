#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Sizing {
    prop: SingleProp("width"),
    is_horizontal: true,
    has_none: false,
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("width"),
    hints: &[],
    matcher: Or(&Length, &Percentage),
});
