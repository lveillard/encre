#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

const fn builtin_plugin(prop: PropertyName) -> Plugin {
    Plugin::AnyNumber {
        prop,
        has_empty: false,
        has_negative: true,
        divide_by: 1.0,
        template: "{}deg",
    }
    // TODO: extra_line CSS_TRANSFORM
}

const fn arbitrary_plugin(prop: PropertyName) -> Plugin {
    Plugin::OnlyArbitrary {
        prop,
        hints: &[],
        matcher: Angle,
    }
    // TODO: extra_line CSS_TRANSFORM
}

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin(SingleProp("--en-skew-x"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin(SingleProp("--en-skew-x"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin(SingleProp("--en-skew-y"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin(SingleProp("--en-skew-y"));
