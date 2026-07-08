#![doc = include_str!("README.md")]
#![doc(alias("spacing", "space"))]
use crate::prelude::build_plugin::*;

const fn plugin(prefix: &'static str, prop: PropertyName) -> (Plugin, Plugin) {
    (
        Plugin::new(PluginKind::Spacing { prefix, prop }).has_auto(),
        Plugin::new(PluginKind::Arbitrary { prefix, prop }),
    )
}

pub(crate) const PLUGIN: (Plugin, Plugin) = plugin("m", SingleProp("margin"));
pub(crate) const PLUGIN_X: (Plugin, Plugin) = plugin("mx", SingleProp("margin-inline"));
pub(crate) const PLUGIN_Y: (Plugin, Plugin) = plugin("my", SingleProp("margin-block"));
pub(crate) const PLUGIN_START: (Plugin, Plugin) = plugin("ms", SingleProp("margin-inline-start"));
pub(crate) const PLUGIN_END: (Plugin, Plugin) = plugin("me", SingleProp("margin-inline-end"));
pub(crate) const PLUGIN_TOP: (Plugin, Plugin) = plugin("mt", SingleProp("margin-top"));
pub(crate) const PLUGIN_BOTTOM: (Plugin, Plugin) = plugin("mb", SingleProp("margin-bottom"));
pub(crate) const PLUGIN_LEFT: (Plugin, Plugin) = plugin("ml", SingleProp("margin-left"));
pub(crate) const PLUGIN_RIGHT: (Plugin, Plugin) = plugin("mr", SingleProp("margin-right"));
