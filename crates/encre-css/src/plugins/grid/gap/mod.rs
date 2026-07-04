#![doc = include_str!("README.md")]
#![doc(alias("grid", "flexbox"))]
use crate::prelude::build_plugin::*;

const fn builtin_plugin(prefix: &'static str, prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Spacing {
        prefix,
        prop,
        has_auto: false,
        has_full: false,
    })
}

const fn arbitrary_plugin(prefix: &'static str, prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Arbitrary { prefix, prop })
}

pub(crate) const PLUGIN_1: Plugin = builtin_plugin("gap", SingleProp("gap"));
pub(crate) const PLUGIN_2: Plugin = arbitrary_plugin("gap", SingleProp("gap"));

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin("gap-x", SingleProp("column-gap"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin("gap-x", SingleProp("column-gap"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin("gap-y", SingleProp("row-gap"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin("gap-y", SingleProp("row-gap"));
