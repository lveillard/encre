#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

const fn builtin_plugin(prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Color { prop })
}

const fn arbitrary_plugin(prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Arbitrary { prop })
        .hints(&[PluginArbitraryHint::Color])
        .matcher(Color)
}

pub(crate) const PLUGIN_1: Plugin = builtin_plugin(SingleProp("border-color"));
pub(crate) const PLUGIN_2: Plugin = arbitrary_plugin(SingleProp("border-color"));

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin(SingleProp("border-inline-color"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin(SingleProp("border-inline-color"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin(SingleProp("border-block-color"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin(SingleProp("border-block-color"));

pub(crate) const PLUGIN_START_1: Plugin = builtin_plugin(SingleProp("border-inline-start-color"));
pub(crate) const PLUGIN_START_2: Plugin = arbitrary_plugin(SingleProp("border-inline-start-color"));

pub(crate) const PLUGIN_END_1: Plugin = builtin_plugin(SingleProp("border-inline-end-color"));
pub(crate) const PLUGIN_END_2: Plugin = arbitrary_plugin(SingleProp("border-inline-end-color"));

pub(crate) const PLUGIN_TOP_1: Plugin = builtin_plugin(SingleProp("border-top-color"));
pub(crate) const PLUGIN_TOP_2: Plugin = arbitrary_plugin(SingleProp("border-top-color"));

pub(crate) const PLUGIN_BOTTOM_1: Plugin = builtin_plugin(SingleProp("border-bottom-color"));
pub(crate) const PLUGIN_BOTTOM_2: Plugin = arbitrary_plugin(SingleProp("border-bottom-color"));

pub(crate) const PLUGIN_LEFT_1: Plugin = builtin_plugin(SingleProp("border-left-color"));
pub(crate) const PLUGIN_LEFT_2: Plugin = arbitrary_plugin(SingleProp("border-left-color"));

pub(crate) const PLUGIN_RIGHT_1: Plugin = builtin_plugin(SingleProp("border-right-color"));
pub(crate) const PLUGIN_RIGHT_2: Plugin = arbitrary_plugin(SingleProp("border-right-color"));
