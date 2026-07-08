#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::{plugins::transform::CSS_TRANSFORM, prelude::build_plugin::*};

type P = (Plugin, Plugin);

const fn plugin(prefix: &'static str, prop: PropertyName) -> P {
    (
        Plugin::new(PluginKind::Number {
            prefix,
            prop,
            divide_by: 1.0,
        })
        .has_negative()
        .extra_lines(&[CSS_TRANSFORM])
        .template("{}deg"),
        Plugin::new(PluginKind::Arbitrary { prefix, prop }).extra_lines(&[CSS_TRANSFORM]),
    )
}

pub(crate) const PLUGIN: P = plugin("rotate", MultipleProps(&["--en-rotate-x", "--en-rotate-y"]));
pub(crate) const PLUGIN_X: P = plugin("rotate-x", SingleProp("--en-rotate-x"));
pub(crate) const PLUGIN_Y: P = plugin("rotate-y", SingleProp("--en-rotate-y"));
pub(crate) const PLUGIN_Z: P = plugin("rotate-z", SingleProp("--en-rotate-z"));
