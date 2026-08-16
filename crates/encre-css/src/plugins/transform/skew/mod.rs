#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::{plugins::transform::CSS_TRANSFORM, prelude::build_plugin::*};

const fn plugin(namespace: &'static str, prop: StaticPropertyName) -> (StaticPlugin, StaticPlugin) {
    (
        Plugin::Number(Number {
            namespace,
            prop,
            has_negative: Some(true),
            template: Some("{}deg"),
            extra_lines: Some(&[CSS_TRANSFORM]),
            ..Number::default()
        }),
        Plugin::Arbitrary(Arbitrary {
            namespace,
            prop,
            extra_lines: Some(&[CSS_TRANSFORM]),
            ..Arbitrary::default()
        }),
    )
}

pub(crate) const PLUGIN_X: (StaticPlugin, StaticPlugin) =
    plugin("skew-x", SingleProp("--en-skew-x"));
pub(crate) const PLUGIN_Y: (StaticPlugin, StaticPlugin) =
    plugin("skew-y", SingleProp("--en-skew-y"));
