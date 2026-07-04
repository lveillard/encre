#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::{plugins::transform::CSS_TRANSFORM, prelude::build_plugin::*};

const fn builtin_plugin(prefix: &'static str, prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::AnyNumber {
        prefix,
        prop,
        has_empty: false,
        has_negative: true,
        divide_by: 1.0,
    })
    .extra_lines(&[CSS_TRANSFORM])
    .template("{}deg")
}

const fn arbitrary_plugin(prefix: &'static str, prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Arbitrary { prefix, prop }).extra_lines(&[CSS_TRANSFORM])
}

pub(crate) const PLUGIN_1: Plugin =
    builtin_plugin("rotate", MultipleProps(&["--en-rotate-x", "--en-rotate-y"]));
pub(crate) const PLUGIN_2: Plugin =
    arbitrary_plugin("rotate", MultipleProps(&["--en-rotate-x", "--en-rotate-y"]));

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin("rotate-x", SingleProp("--en-rotate-x"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin("rotate-x", SingleProp("--en-rotate-x"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin("rotate-y", SingleProp("--en-rotate-y"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin("rotate-y", SingleProp("--en-rotate-y"));

pub(crate) const PLUGIN_Z_1: Plugin = builtin_plugin("rotate-z", SingleProp("--en-rotate-z"));
pub(crate) const PLUGIN_Z_2: Plugin = arbitrary_plugin("rotate-z", SingleProp("--en-rotate-z"));
