#![doc = include_str!("README.md")]
#![doc(alias("layout", "inset"))]
use crate::prelude::build_plugin::*;

const fn plugin(prefix: &'static str, prop: PropertyName) -> (Plugin, Plugin) {
    (
        Plugin::new(PluginKind::Spacing {
            prefix,
            prop,
            has_auto: true,
            has_full: true,
        }),
        Plugin::new(PluginKind::Arbitrary { prefix, prop }),
    )
}

pub(crate) const PLUGIN: (Plugin, Plugin) = plugin("inset", SingleProp("inset"));
pub(crate) const PLUGIN_X: (Plugin, Plugin) = plugin("inset-x", SingleProp("inset-inline"));
pub(crate) const PLUGIN_Y: (Plugin, Plugin) = plugin("inset-y", SingleProp("inset-block"));
pub(crate) const PLUGIN_START: (Plugin, Plugin) = plugin("start", SingleProp("inset-inline-start"));
pub(crate) const PLUGIN_END: (Plugin, Plugin) = plugin("end", SingleProp("inset-inline-end"));
pub(crate) const PLUGIN_TOP: (Plugin, Plugin) = plugin("top", SingleProp("top"));
pub(crate) const PLUGIN_BOTTOM: (Plugin, Plugin) = plugin("bottom", SingleProp("bottom"));
pub(crate) const PLUGIN_LEFT: (Plugin, Plugin) = plugin("left", SingleProp("left"));
pub(crate) const PLUGIN_RIGHT: (Plugin, Plugin) = plugin("right", SingleProp("right"));
