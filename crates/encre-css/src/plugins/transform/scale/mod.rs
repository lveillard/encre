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
            divide_by: Some(100.0),
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

pub(crate) const PLUGIN: P = plugin("scale", MultipleProps(&["--en-scale-x", "--en-scale-y"]));
pub(crate) const PLUGIN_X: P = plugin("scale-x", SingleProp("--en-scale-x"));
pub(crate) const PLUGIN_Y: P = plugin("scale-y", SingleProp("--en-scale-y"));
pub(crate) const PLUGIN_Z: P = plugin("scale-z", SingleProp("--en-scale-z"));
