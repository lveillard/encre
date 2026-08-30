#![doc = include_str!("README.md")]
#![doc(alias("spacing", "space"))]
use crate::prelude::build_plugin::*;

const fn plugin(namespace: &'static str, prop: StaticPropertyName) -> (StaticPlugin, StaticPlugin) {
    (
        Plugin::Spacing(Spacing {
            namespace,
            prop,
            has_auto: Some(true),
            ..Spacing::default()
        }),
        Plugin::Arbitrary(Arbitrary {
            namespace,
            prop,
            ..Arbitrary::default()
        }),
    )
}

pub(crate) const PLUGIN: (StaticPlugin, StaticPlugin) = plugin("m", SingleProp("margin"));
pub(crate) const PLUGIN_X: (StaticPlugin, StaticPlugin) = plugin("mx", SingleProp("margin-inline"));
pub(crate) const PLUGIN_Y: (StaticPlugin, StaticPlugin) = plugin("my", SingleProp("margin-block"));
pub(crate) const PLUGIN_START: (StaticPlugin, StaticPlugin) =
    plugin("ms", SingleProp("margin-inline-start"));
pub(crate) const PLUGIN_END: (StaticPlugin, StaticPlugin) =
    plugin("me", SingleProp("margin-inline-end"));
pub(crate) const PLUGIN_TOP: (StaticPlugin, StaticPlugin) = plugin("mt", SingleProp("margin-top"));
pub(crate) const PLUGIN_BOTTOM: (StaticPlugin, StaticPlugin) =
    plugin("mb", SingleProp("margin-bottom"));
pub(crate) const PLUGIN_LEFT: (StaticPlugin, StaticPlugin) =
    plugin("ml", SingleProp("margin-left"));
pub(crate) const PLUGIN_RIGHT: (StaticPlugin, StaticPlugin) =
    plugin("mr", SingleProp("margin-right"));
