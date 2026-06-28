#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::Spacing {
    prop: SingleProp("flex-basis"),
    has_auto: true,
    has_full: true,
};

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("flex-basis"),
    hints: &[],
    matcher: Length,
};
