#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

type P = (StaticPlugin, StaticPlugin);

const fn plugin(namespace: &'static str, prop: StaticPropertyName) -> P {
    (
        Plugin::Number(Number {
            namespace,
            prop,
            has_empty: Some(true),
            template: Some("{}px"),
            ..Number::default()
        }),
        Plugin::Arbitrary(Arbitrary {
            namespace,
            prop,
            hints: Some(&[ArbitraryHint::Length, ArbitraryHint::LineWidth]),
            matchers: Some((
                &[
                    PluginArbitraryMatcher::Length,
                    PluginArbitraryMatcher::LineWidth,
                ],
                PluginArbitraryMatcherSeparation::Space,
            )),
            ..Arbitrary::default()
        }),
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
