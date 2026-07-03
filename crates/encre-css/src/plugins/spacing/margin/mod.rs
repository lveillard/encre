#![doc = include_str!("README.md")]
#![doc(alias("spacing", "space"))]
use crate::prelude::build_plugin::*;

const fn builtin_plugin(prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Spacing {
        prop,
        has_auto: true,
        has_full: false,
    })
}

const fn arbitrary_plugin(prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Arbitrary {
        prop,
    })
}

pub(crate) const PLUGIN_1: Plugin = builtin_plugin(SingleProp("margin"));
pub(crate) const PLUGIN_2: Plugin = arbitrary_plugin(SingleProp("margin"));

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin(SingleProp("margin-inline"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin(SingleProp("margin-inline"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin(SingleProp("margin-block"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin(SingleProp("margin-block"));

pub(crate) const PLUGIN_START_1: Plugin = builtin_plugin(SingleProp("margin-inline-start"));
pub(crate) const PLUGIN_START_2: Plugin = arbitrary_plugin(SingleProp("margin-inline-start"));

pub(crate) const PLUGIN_END_1: Plugin = builtin_plugin(SingleProp("margin-inline-end"));
pub(crate) const PLUGIN_END_2: Plugin = arbitrary_plugin(SingleProp("margin-inline-end"));

pub(crate) const PLUGIN_TOP_1: Plugin = builtin_plugin(SingleProp("margin-top"));
pub(crate) const PLUGIN_TOP_2: Plugin = arbitrary_plugin(SingleProp("margin-top"));

pub(crate) const PLUGIN_BOTTOM_1: Plugin = builtin_plugin(SingleProp("margin-bottom"));
pub(crate) const PLUGIN_BOTTOM_2: Plugin = arbitrary_plugin(SingleProp("margin-bottom"));

pub(crate) const PLUGIN_LEFT_1: Plugin = builtin_plugin(SingleProp("margin-left"));
pub(crate) const PLUGIN_LEFT_2: Plugin = arbitrary_plugin(SingleProp("margin-left"));

pub(crate) const PLUGIN_RIGHT_1: Plugin = builtin_plugin(SingleProp("margin-right"));
pub(crate) const PLUGIN_RIGHT_2: Plugin = arbitrary_plugin(SingleProp("margin-right"));
