#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::Sizing {
    prop: SingleProp("min-height"),
    is_horizontal: false,
    has_none: false,
};

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("min-height"),
    hints: &[],
    matcher: Or(&Length, &Percentage),
};
