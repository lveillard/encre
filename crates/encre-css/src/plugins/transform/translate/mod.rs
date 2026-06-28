#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

const fn builtin_plugin(prop: PropertyName) -> Plugin {
    Plugin::Spacing {
        prop,
        has_auto: true,
        has_full: true,
    }
    // TODO: extra_line CSS_TRANSFORM
}

const fn arbitrary_plugin(prop: PropertyName) -> Plugin {
    Plugin::OnlyArbitrary {
        prop,
        hints: &[],
        matcher: Or(&Length, &Percentage),
    }
    // TODO: extra_line CSS_TRANSFORM
}

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin(SingleProp("--en-translate-x"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin(SingleProp("--en-translate-x"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin(SingleProp("--en-translate-y"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin(SingleProp("--en-translate-y"));

pub(crate) const PLUGIN_Z_1: Plugin = builtin_plugin(SingleProp("--en-translate-z"));
pub(crate) const PLUGIN_Z_2: Plugin = arbitrary_plugin(SingleProp("--en-translate-z"));
