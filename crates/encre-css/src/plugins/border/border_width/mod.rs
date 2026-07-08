#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

type P = (Plugin, Plugin);

const fn plugin(prefix: &'static str, prop: PropertyName) -> P {
    (
        Plugin::new(PluginKind::Number {
            prefix,
            prop,
            divide_by: 1.0,
        })
        .has_empty()
        .template("{}px"),
        Plugin::new(PluginKind::Arbitrary { prefix, prop })
            .hints(&[PluginArbitraryHint::Length, PluginArbitraryHint::LineWidth])
            .matcher(SpaceSeparated(&Or(&Length, &LineWidth))),
    )
}

pub(crate) const PLUGIN: P = plugin("border", SingleProp("border-width"));
pub(crate) const PLUGIN_X: P = plugin("border-x", SingleProp("border-inline-width"));
pub(crate) const PLUGIN_Y: P = plugin("border-y", SingleProp("border-block-width"));
pub(crate) const PLUGIN_START: P = plugin("border-s", SingleProp("border-inline-start-width"));
pub(crate) const PLUGIN_END: P = plugin("border-e", SingleProp("border-inline-end-width"));
pub(crate) const PLUGIN_TOP: P = plugin("border-t", SingleProp("border-top-width"));
pub(crate) const PLUGIN_BOTTOM: P = plugin("border-b", SingleProp("border-bottom-width"));
pub(crate) const PLUGIN_LEFT: P = plugin("border-l", SingleProp("border-left-width"));
pub(crate) const PLUGIN_RIGHT: P = plugin("border-r", SingleProp("border-right-width"));
