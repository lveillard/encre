#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::Spacing {
    prop: SingleProp("text-indent"),
    has_auto: false,
    has_full: false,
};

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("text-indent"),
    hints: &[],
    matcher: Length,
};
