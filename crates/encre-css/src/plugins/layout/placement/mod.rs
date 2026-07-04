#![doc = include_str!("README.md")]
#![doc(alias("layout", "inset"))]
use crate::prelude::build_plugin::*;

const fn builtin_plugin(prefix: &'static str, prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Spacing {
        prefix,
        prop,
        has_auto: true,
        has_full: true,
    })
}

const fn arbitrary_plugin(prefix: &'static str, prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Arbitrary { prefix, prop })
}

pub(crate) const PLUGIN_1: Plugin = builtin_plugin("inset", SingleProp("inset"));
pub(crate) const PLUGIN_2: Plugin = arbitrary_plugin("inset", SingleProp("inset"));

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin("inset-x", SingleProp("inset-inline"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin("inset-x", SingleProp("inset-inline"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin("inset-y", SingleProp("inset-block"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin("inset-y", SingleProp("inset-block"));

pub(crate) const PLUGIN_START_1: Plugin = builtin_plugin("start", SingleProp("inset-inline-start"));
pub(crate) const PLUGIN_START_2: Plugin = arbitrary_plugin("start", SingleProp("inset-inline-start"));

pub(crate) const PLUGIN_END_1: Plugin = builtin_plugin("end", SingleProp("inset-inline-end"));
pub(crate) const PLUGIN_END_2: Plugin = arbitrary_plugin("end", SingleProp("inset-inline-end"));

pub(crate) const PLUGIN_TOP_1: Plugin = builtin_plugin("top", SingleProp("top"));
pub(crate) const PLUGIN_TOP_2: Plugin = arbitrary_plugin("top", SingleProp("top"));

pub(crate) const PLUGIN_BOTTOM_1: Plugin = builtin_plugin("bottom", SingleProp("bottom"));
pub(crate) const PLUGIN_BOTTOM_2: Plugin = arbitrary_plugin("bottom", SingleProp("bottom"));

pub(crate) const PLUGIN_LEFT_1: Plugin = builtin_plugin("left", SingleProp("left"));
pub(crate) const PLUGIN_LEFT_2: Plugin = arbitrary_plugin("left", SingleProp("left"));

pub(crate) const PLUGIN_RIGHT_1: Plugin = builtin_plugin("right", SingleProp("right"));
pub(crate) const PLUGIN_RIGHT_2: Plugin = arbitrary_plugin("right", SingleProp("right"));
