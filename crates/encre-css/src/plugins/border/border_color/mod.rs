#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

type P = (StaticPlugin, StaticPlugin);

const fn plugin(namespace: &'static str, prop: StaticPropertyName) -> P {
    (
        Plugin::new(PluginKind::Color { namespace, prop }),
        Plugin::new(PluginKind::Arbitrary { namespace, prop })
            .hints(&[ArbitraryHint::Color])
            .matchers(&[Color], PluginArbitraryMatcherModifier::None),
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
