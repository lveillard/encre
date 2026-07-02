#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::{plugins::transform::CSS_TRANSFORM, prelude::build_plugin::*};

const fn builtin_plugin(prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Spacing {
        prop,
        has_auto: true,
        has_full: true,
    })
    .extra_lines(&[CSS_TRANSFORM])
}

const fn arbitrary_plugin(prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::OnlyArbitrary {
        prop,
        hints: &[],
        matcher: Or(&Length, &Percentage),
    })
    .extra_lines(&[CSS_TRANSFORM])
}

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin(SingleProp("--en-translate-x"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin(SingleProp("--en-translate-x"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin(SingleProp("--en-translate-y"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin(SingleProp("--en-translate-y"));

pub(crate) const PLUGIN_Z_1: Plugin = builtin_plugin(SingleProp("--en-translate-z"));
pub(crate) const PLUGIN_Z_2: Plugin = arbitrary_plugin(SingleProp("--en-translate-z"));
