#![doc = include_str!("README.md")]
#![doc(alias("spacing", "space"))]
use crate::prelude::build_plugin::*;

const fn builtin_plugin(prefix: &'static str, prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Spacing {
        prefix,
        prop,
        has_auto: true,
        has_full: false,
    })
}

const fn arbitrary_plugin(prefix: &'static str, prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Arbitrary {
        prefix,
        prop,
    })
}

pub(crate) const PLUGIN_1: Plugin = builtin_plugin("p", SingleProp("padding"));
pub(crate) const PLUGIN_2: Plugin = arbitrary_plugin("p", SingleProp("padding"));

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin("px", SingleProp("padding-inline"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin("px", SingleProp("padding-inline"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin("py", SingleProp("padding-block"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin("py", SingleProp("padding-block"));

pub(crate) const PLUGIN_START_1: Plugin = builtin_plugin("ps", SingleProp("padding-inline-start"));
pub(crate) const PLUGIN_START_2: Plugin = arbitrary_plugin("ps", SingleProp("padding-inline-start"));

pub(crate) const PLUGIN_END_1: Plugin = builtin_plugin("pe", SingleProp("padding-inline-end"));
pub(crate) const PLUGIN_END_2: Plugin = arbitrary_plugin("pe", SingleProp("padding-inline-end"));

pub(crate) const PLUGIN_TOP_1: Plugin = builtin_plugin("pt", SingleProp("padding-top"));
pub(crate) const PLUGIN_TOP_2: Plugin = arbitrary_plugin("pt", SingleProp("padding-top"));

pub(crate) const PLUGIN_BOTTOM_1: Plugin = builtin_plugin("pb", SingleProp("padding-bottom"));
pub(crate) const PLUGIN_BOTTOM_2: Plugin = arbitrary_plugin("pb", SingleProp("padding-bottom"));

pub(crate) const PLUGIN_LEFT_1: Plugin = builtin_plugin("pl", SingleProp("padding-left"));
pub(crate) const PLUGIN_LEFT_2: Plugin = arbitrary_plugin("pl", SingleProp("padding-left"));

pub(crate) const PLUGIN_RIGHT_1: Plugin = builtin_plugin("pr", SingleProp("padding-right"));
pub(crate) const PLUGIN_RIGHT_2: Plugin = arbitrary_plugin("pr", SingleProp("padding-right"));
