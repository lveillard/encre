#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::{plugins::transform::CSS_TRANSFORM, prelude::build_plugin::*};

type P = (StaticPlugin, StaticPlugin);

const fn plugin(namespace: &'static str, prop: StaticPropertyName) -> P {
    (
        Plugin::Number(Number {
            namespace,
            prop,
            has_negative: Some(true),
            extra_lines: Some(&[CSS_TRANSFORM]),
            template: Some("{}deg"),
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

pub(crate) const PLUGIN: P = plugin("rotate", MultipleProps(&["--en-rotate-x", "--en-rotate-y"]));
pub(crate) const PLUGIN_X: P = plugin("rotate-x", SingleProp("--en-rotate-x"));
pub(crate) const PLUGIN_Y: P = plugin("rotate-y", SingleProp("--en-rotate-y"));
pub(crate) const PLUGIN_Z: P = plugin("rotate-z", SingleProp("--en-rotate-z"));
