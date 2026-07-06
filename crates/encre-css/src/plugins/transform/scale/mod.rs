#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::{plugins::transform::CSS_TRANSFORM, prelude::build_plugin::*};

type P = (Plugin, Plugin);

const fn plugin(prefix: &'static str, prop: PropertyName) -> P {
    (
        Plugin::new(PluginKind::AnyNumber {
            prefix,
            prop,
            has_empty: false,
            has_negative: true,
            divide_by: 100.0,
        })
        .extra_lines(&[CSS_TRANSFORM]),
        Plugin::new(PluginKind::Arbitrary { prefix, prop }).extra_lines(&[CSS_TRANSFORM]),
    )
}

pub(crate) const PLUGIN: P = plugin("scale", MultipleProps(&["--en-scale-x", "--en-scale-y"]));
pub(crate) const PLUGIN_X: P = plugin("scale-x", SingleProp("--en-scale-x"));
pub(crate) const PLUGIN_Y: P = plugin("scale-y", SingleProp("--en-scale-y"));
pub(crate) const PLUGIN_Z: P = plugin("scale-z", SingleProp("--en-scale-z"));
