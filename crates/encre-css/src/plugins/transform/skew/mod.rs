#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::{plugins::transform::CSS_TRANSFORM, prelude::build_plugin::*};

const fn plugin(prefix: &'static str, prop: PropertyName) -> (Plugin, Plugin) {
    (
        Plugin::new(PluginKind::AnyNumber {
            prefix,
            prop,
            has_empty: false,
            has_negative: true,
            divide_by: 1.0,
        })
        .extra_lines(&[CSS_TRANSFORM])
        .template("{}deg"),
        Plugin::new(PluginKind::Arbitrary { prefix, prop }).extra_lines(&[CSS_TRANSFORM]),
    )
}

pub(crate) const PLUGIN_X: (Plugin, Plugin) = plugin("skew-x", SingleProp("--en-skew-x"));
pub(crate) const PLUGIN_Y: (Plugin, Plugin) = plugin("skew-y", SingleProp("--en-skew-y"));
