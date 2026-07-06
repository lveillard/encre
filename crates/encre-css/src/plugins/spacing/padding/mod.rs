#![doc = include_str!("README.md")]
#![doc(alias("spacing", "space"))]
use crate::prelude::build_plugin::*;

const fn plugin(prefix: &'static str, prop: PropertyName) -> (Plugin, Plugin) {
    (
        Plugin::new(PluginKind::Spacing {
            prefix,
            prop,
            has_auto: true,
            has_full: false,
        }),
        Plugin::new(PluginKind::Arbitrary { prefix, prop }),
    )
}

pub(crate) const PLUGIN: (Plugin, Plugin) = plugin("p", SingleProp("padding"));
pub(crate) const PLUGIN_X: (Plugin, Plugin) = plugin("px", SingleProp("padding-inline"));
pub(crate) const PLUGIN_Y: (Plugin, Plugin) = plugin("py", SingleProp("padding-block"));
pub(crate) const PLUGIN_START: (Plugin, Plugin) = plugin("ps", SingleProp("padding-inline-start"));
pub(crate) const PLUGIN_END: (Plugin, Plugin) = plugin("pe", SingleProp("padding-inline-end"));
pub(crate) const PLUGIN_TOP: (Plugin, Plugin) = plugin("pt", SingleProp("padding-top"));
pub(crate) const PLUGIN_BOTTOM: (Plugin, Plugin) = plugin("pb", SingleProp("padding-bottom"));
pub(crate) const PLUGIN_LEFT: (Plugin, Plugin) = plugin("pl", SingleProp("padding-left"));
pub(crate) const PLUGIN_RIGHT: (Plugin, Plugin) = plugin("pr", SingleProp("padding-right"));
