#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::{plugins::transform::CSS_TRANSFORM, prelude::build_plugin::*};

const fn builtin_plugin(prefix: &'static str, prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Spacing {
        prefix,
        prop,
        has_auto: true,
        has_full: true,
    })
    .extra_lines(&[CSS_TRANSFORM])
}

const fn arbitrary_plugin(prefix: &'static str, prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Arbitrary { prefix, prop }).extra_lines(&[CSS_TRANSFORM])
}

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin("translate-x", SingleProp("--en-translate-x"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin("translate-x", SingleProp("--en-translate-x"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin("translate-y", SingleProp("--en-translate-y"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin("translate-y", SingleProp("--en-translate-y"));

pub(crate) const PLUGIN_Z_1: Plugin = builtin_plugin("translate-z", SingleProp("--en-translate-z"));
pub(crate) const PLUGIN_Z_2: Plugin = arbitrary_plugin("translate-z", SingleProp("--en-translate-z"));
