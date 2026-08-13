#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::{plugins::transform::CSS_TRANSFORM, prelude::build_plugin::*};

const fn plugin(namespace: &'static str, prop: StaticPropertyName) -> (StaticPlugin, StaticPlugin) {
    (
        Plugin::new(PluginKind::Number {
            namespace,
            prop,
        })
        .has_negative()
        .extra_lines(&[CSS_TRANSFORM])
        .template("{}deg"),
        Plugin::new(PluginKind::Arbitrary { namespace, prop }).extra_lines(&[CSS_TRANSFORM]),
    )
}

pub(crate) const PLUGIN_X: (StaticPlugin, StaticPlugin) = plugin("skew-x", SingleProp("--en-skew-x"));
pub(crate) const PLUGIN_Y: (StaticPlugin, StaticPlugin) = plugin("skew-y", SingleProp("--en-skew-y"));
