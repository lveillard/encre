#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::{plugins::transform::CSS_TRANSFORM, prelude::build_plugin::*};

const fn plugin(namespace: &'static str, prop: StaticPropertyName) -> (StaticPlugin, StaticPlugin) {
    (
        Plugin::Spacing(Spacing {
            namespace,
            prop,
            has_auto: Some(true),
            has_full: Some(true),
            extra_rule_css: Some(&[CSS_TRANSFORM]),
            ..Spacing::default()
        }),
        Plugin::Arbitrary(Arbitrary {
            namespace,
            prop,
            extra_rule_css: Some(&[CSS_TRANSFORM]),
            ..Arbitrary::default()
        }),
    )
}

pub(crate) const PLUGIN_X: (StaticPlugin, StaticPlugin) =
    plugin("translate-x", SingleProp("--en-translate-x"));
pub(crate) const PLUGIN_Y: (StaticPlugin, StaticPlugin) =
    plugin("translate-y", SingleProp("--en-translate-y"));
pub(crate) const PLUGIN_Z: (StaticPlugin, StaticPlugin) =
    plugin("translate-z", SingleProp("--en-translate-z"));
