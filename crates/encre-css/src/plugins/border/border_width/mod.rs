#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

const fn builtin_plugin(prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::AnyNumber {
        prop,
        has_empty: true,
        has_negative: false,
        divide_by: 1.0,
    })
    .template("{}px")
}

const fn arbitrary_plugin(prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Arbitrary { prop })
        .hints(&[PluginArbitraryHint::Length, PluginArbitraryHint::LineWidth])
        .matcher(SpaceSeparated(&Or(&Length, &LineWidth)))
}

pub(crate) const PLUGIN_1: Plugin = builtin_plugin(SingleProp("border-width"));
pub(crate) const PLUGIN_2: Plugin = arbitrary_plugin(SingleProp("border-width"));

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin(SingleProp("border-inline-width"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin(SingleProp("border-inline-width"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin(SingleProp("border-block-width"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin(SingleProp("border-block-width"));

pub(crate) const PLUGIN_START_1: Plugin = builtin_plugin(SingleProp("border-inline-start-width"));
pub(crate) const PLUGIN_START_2: Plugin = arbitrary_plugin(SingleProp("border-inline-start-width"));

pub(crate) const PLUGIN_END_1: Plugin = builtin_plugin(SingleProp("border-inline-end-width"));
pub(crate) const PLUGIN_END_2: Plugin = arbitrary_plugin(SingleProp("border-inline-end-width"));

pub(crate) const PLUGIN_TOP_1: Plugin = builtin_plugin(SingleProp("border-top-width"));
pub(crate) const PLUGIN_TOP_2: Plugin = arbitrary_plugin(SingleProp("border-top-width"));

pub(crate) const PLUGIN_BOTTOM_1: Plugin = builtin_plugin(SingleProp("border-bottom-width"));
pub(crate) const PLUGIN_BOTTOM_2: Plugin = arbitrary_plugin(SingleProp("border-bottom-width"));

pub(crate) const PLUGIN_LEFT_1: Plugin = builtin_plugin(SingleProp("border-left-width"));
pub(crate) const PLUGIN_LEFT_2: Plugin = arbitrary_plugin(SingleProp("border-left-width"));

pub(crate) const PLUGIN_RIGHT_1: Plugin = builtin_plugin(SingleProp("border-right-width"));
pub(crate) const PLUGIN_RIGHT_2: Plugin = arbitrary_plugin(SingleProp("border-right-width"));
