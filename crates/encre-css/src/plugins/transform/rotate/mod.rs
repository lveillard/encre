#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::{plugins::transform::CSS_TRANSFORM, prelude::build_plugin::*};
use PluginArbitraryMatcher::*;

const fn builtin_plugin(prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::AnyNumber {
        prop,
        has_empty: false,
        has_negative: true,
        divide_by: 1.0,
    }).extra_lines(&[CSS_TRANSFORM]).template("{}deg")
}

const fn arbitrary_plugin(prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::OnlyArbitrary {
        prop,
        hints: &[],
        matcher: Angle,
    }).extra_lines(&[CSS_TRANSFORM])
}

pub(crate) const PLUGIN_1: Plugin =
    builtin_plugin(MultipleProps(&["--en-rotate-x", "--en-rotate-y"]));
pub(crate) const PLUGIN_2: Plugin =
    arbitrary_plugin(MultipleProps(&["--en-rotate-x", "--en-rotate-y"]));

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin(SingleProp("--en-rotate-x"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin(SingleProp("--en-rotate-x"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin(SingleProp("--en-rotate-y"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin(SingleProp("--en-rotate-y"));

pub(crate) const PLUGIN_Z_1: Plugin = builtin_plugin(SingleProp("--en-rotate-z"));
pub(crate) const PLUGIN_Z_2: Plugin = arbitrary_plugin(SingleProp("--en-rotate-z"));
