#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
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
    Plugin::new(PluginKind::Arbitrary {
        prefix,
        prop,
    })
}

pub(crate) const PLUGIN_1: Plugin = builtin_plugin("scroll-m", SingleProp("scroll-margin"));
pub(crate) const PLUGIN_2: Plugin = arbitrary_plugin("scroll-m", SingleProp("scroll-margin"));

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin("scroll-mx", SingleProp("scroll-margin-inline"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin("scroll-mx", SingleProp("scroll-margin-inline"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin("scroll-my", SingleProp("scroll-margin-block"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin("scroll-my", SingleProp("scroll-margin-block"));

pub(crate) const PLUGIN_START_1: Plugin = builtin_plugin("scroll-ms", SingleProp("scroll-margin-inline-start"));
pub(crate) const PLUGIN_START_2: Plugin = arbitrary_plugin("scroll-ms", SingleProp("scroll-margin-inline-start"));

pub(crate) const PLUGIN_END_1: Plugin = builtin_plugin("scroll-me", SingleProp("scroll-margin-inline-end"));
pub(crate) const PLUGIN_END_2: Plugin = arbitrary_plugin("scroll-me", SingleProp("scroll-margin-inline-end"));

pub(crate) const PLUGIN_TOP_1: Plugin = builtin_plugin("scroll-mt", SingleProp("scroll-margin-top"));
pub(crate) const PLUGIN_TOP_2: Plugin = arbitrary_plugin("scroll-mt", SingleProp("scroll-margin-top"));

pub(crate) const PLUGIN_BOTTOM_1: Plugin = builtin_plugin("scroll-mb", SingleProp("scroll-margin-bottom"));
pub(crate) const PLUGIN_BOTTOM_2: Plugin = arbitrary_plugin("scroll-mb", SingleProp("scroll-margin-bottom"));

pub(crate) const PLUGIN_LEFT_1: Plugin = builtin_plugin("scroll-ml", SingleProp("scroll-margin-left"));
pub(crate) const PLUGIN_LEFT_2: Plugin = arbitrary_plugin("scroll-ml", SingleProp("scroll-margin-left"));

pub(crate) const PLUGIN_RIGHT_1: Plugin = builtin_plugin("scroll-mr", SingleProp("scroll-margin-right"));
pub(crate) const PLUGIN_RIGHT_2: Plugin = arbitrary_plugin("scroll-mr", SingleProp("scroll-margin-right"));
