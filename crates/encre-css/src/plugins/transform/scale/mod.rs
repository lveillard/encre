#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::{plugins::transform::CSS_TRANSFORM, prelude::build_plugin::*};

const fn builtin_plugin(prefix: &'static str, prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::AnyNumber {
        prefix,
        prop,
        has_empty: false,
        has_negative: true,
        divide_by: 100.0,
    })
    .extra_lines(&[CSS_TRANSFORM])
}

const fn arbitrary_plugin(prefix: &'static str, prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Arbitrary {
        prefix,
        prop,
    })
    .extra_lines(&[CSS_TRANSFORM])
}

pub(crate) const PLUGIN_1: Plugin =
    builtin_plugin("scale", MultipleProps(&["--en-scale-x", "--en-scale-y"]));
pub(crate) const PLUGIN_2: Plugin =
    arbitrary_plugin("scale", MultipleProps(&["--en-scale-x", "--en-scale-y"]));

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin("scale-x", SingleProp("--en-scale-x"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin("scale-x", SingleProp("--en-scale-x"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin("scale-y", SingleProp("--en-scale-y"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin("scale-y", SingleProp("--en-scale-y"));

pub(crate) const PLUGIN_Z_1: Plugin = builtin_plugin("scale-z", SingleProp("--en-scale-z"));
pub(crate) const PLUGIN_Z_2: Plugin = arbitrary_plugin("scale-z", SingleProp("--en-scale-z"));
