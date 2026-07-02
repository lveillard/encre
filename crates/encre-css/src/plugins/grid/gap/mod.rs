#![doc = include_str!("README.md")]
#![doc(alias("grid", "flexbox"))]
use crate::prelude::build_plugin::*;

const fn builtin_plugin(prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Spacing {
        prop,
        has_auto: false,
        has_full: false,
    })
}

const fn arbitrary_plugin(prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Arbitrary {
        prop,
        hints: &[],
        matcher: Length,
    })
}

pub(crate) const PLUGIN_1: Plugin = builtin_plugin(SingleProp("gap"));
pub(crate) const PLUGIN_2: Plugin = arbitrary_plugin(SingleProp("gap"));

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin(SingleProp("column-gap"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin(SingleProp("column-gap"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin(SingleProp("row-gap"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin(SingleProp("row-gap"));
