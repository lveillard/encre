#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::{plugins::transform::CSS_TRANSFORM, prelude::build_plugin::*};

const fn plugin(prefix: &'static str, prop: PropertyName) -> (Plugin, Plugin) {
    (
        Plugin::new(PluginKind::Spacing {
            prefix,
            prop,
            has_auto: true,
            has_full: true,
        })
        .extra_lines(&[CSS_TRANSFORM]),
        Plugin::new(PluginKind::Arbitrary { prefix, prop }).extra_lines(&[CSS_TRANSFORM]),
    )
}

pub(crate) const PLUGIN_X: (Plugin, Plugin) = plugin("translate-x", SingleProp("--en-translate-x"));
pub(crate) const PLUGIN_Y: (Plugin, Plugin) = plugin("translate-y", SingleProp("--en-translate-y"));
pub(crate) const PLUGIN_Z: (Plugin, Plugin) = plugin("translate-z", SingleProp("--en-translate-z"));
