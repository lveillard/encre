#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

const fn builtin_plugin(prop: PropertyName) -> Plugin {
    Plugin::Spacing {
        prop,
        has_auto: false,
        has_full: false,
    }
}

const fn arbitrary_plugin(prop: PropertyName) -> Plugin {
    Plugin::OnlyArbitrary {
        prop,
        hints: &[],
        matcher: Length,
    }
}

pub(crate) const PLUGIN_1: Plugin = builtin_plugin(SingleProp("scroll-padding"));
pub(crate) const PLUGIN_2: Plugin = arbitrary_plugin(SingleProp("scroll-padding"));

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin(SingleProp("scroll-padding-inline"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin(SingleProp("scroll-padding-inline"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin(SingleProp("scroll-padding-block"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin(SingleProp("scroll-padding-block"));

pub(crate) const PLUGIN_START_1: Plugin = builtin_plugin(SingleProp("scroll-padding-inline-start"));
pub(crate) const PLUGIN_START_2: Plugin = arbitrary_plugin(SingleProp("scroll-padding-inline-start"));

pub(crate) const PLUGIN_END_1: Plugin = builtin_plugin(SingleProp("scroll-padding-inline-end"));
pub(crate) const PLUGIN_END_2: Plugin = arbitrary_plugin(SingleProp("scroll-padding-inline-end"));

pub(crate) const PLUGIN_TOP_1: Plugin = builtin_plugin(SingleProp("scroll-padding-top"));
pub(crate) const PLUGIN_TOP_2: Plugin = arbitrary_plugin(SingleProp("scroll-padding-top"));

pub(crate) const PLUGIN_BOTTOM_1: Plugin = builtin_plugin(SingleProp("scroll-padding-bottom"));
pub(crate) const PLUGIN_BOTTOM_2: Plugin = arbitrary_plugin(SingleProp("scroll-padding-bottom"));

pub(crate) const PLUGIN_LEFT_1: Plugin = builtin_plugin(SingleProp("scroll-padding-left"));
pub(crate) const PLUGIN_LEFT_2: Plugin = arbitrary_plugin(SingleProp("scroll-padding-left"));

pub(crate) const PLUGIN_RIGHT_1: Plugin = builtin_plugin(SingleProp("scroll-padding-right"));
pub(crate) const PLUGIN_RIGHT_2: Plugin = arbitrary_plugin(SingleProp("scroll-padding-right"));
