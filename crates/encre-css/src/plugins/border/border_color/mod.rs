#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

type P = (Plugin, Plugin);

const fn plugin(prefix: &'static str, prop: PropertyName) -> P {
    (
        Plugin::new(PluginKind::Color { prefix, prop }),
        Plugin::new(PluginKind::Arbitrary { prefix, prop })
            .hints(&[ArbitraryHint::Color])
            .matcher(Color),
    )
}

pub(crate) const PLUGIN: P = plugin("border", SingleProp("border-color"));
pub(crate) const PLUGIN_X: P = plugin("border-x", SingleProp("border-inline-color"));
pub(crate) const PLUGIN_Y: P = plugin("border-y", SingleProp("border-block-color"));
pub(crate) const PLUGIN_START: P = plugin("border-s", SingleProp("border-inline-start-color"));
pub(crate) const PLUGIN_END: P = plugin("border-e", SingleProp("border-inline-end-color"));
pub(crate) const PLUGIN_TOP: P = plugin("border-t", SingleProp("border-top-color"));
pub(crate) const PLUGIN_BOTTOM: P = plugin("border-b", SingleProp("border-bottom-color"));
pub(crate) const PLUGIN_LEFT: P = plugin("border-l", SingleProp("border-left-color"));
pub(crate) const PLUGIN_RIGHT: P = plugin("border-r", SingleProp("border-right-color"));
