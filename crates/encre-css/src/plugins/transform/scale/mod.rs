#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::{plugins::transform::CSS_TRANSFORM, prelude::build_plugin::*};

const fn builtin_plugin(prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::AnyNumber {
        prop,
        has_empty: false,
        has_negative: true,
        divide_by: 100.0,
    })
    .extra_lines(&[CSS_TRANSFORM])
}

const fn arbitrary_plugin(prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Arbitrary {
        prop,
    })
    .extra_lines(&[CSS_TRANSFORM])
}

pub(crate) const PLUGIN_1: Plugin =
    builtin_plugin(MultipleProps(&["--en-scale-x", "--en-scale-y"]));
pub(crate) const PLUGIN_2: Plugin =
    arbitrary_plugin(MultipleProps(&["--en-scale-x", "--en-scale-y"]));

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin(SingleProp("--en-scale-x"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin(SingleProp("--en-scale-x"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin(SingleProp("--en-scale-y"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin(SingleProp("--en-scale-y"));

pub(crate) const PLUGIN_Z_1: Plugin = builtin_plugin(SingleProp("--en-scale-z"));
pub(crate) const PLUGIN_Z_2: Plugin = arbitrary_plugin(SingleProp("--en-scale-z"));
