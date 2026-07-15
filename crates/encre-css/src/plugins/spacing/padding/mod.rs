#![doc = include_str!("README.md")]
#![doc(alias("spacing", "space"))]
use crate::prelude::build_plugin::*;

const fn plugin(namespace: &'static str, prop: StaticPropertyName) -> (StaticPlugin, StaticPlugin) {
    (
        Plugin::new(PluginKind::Spacing { namespace, prop }).has_auto(),
        Plugin::new(PluginKind::Arbitrary { namespace, prop }),
    )
}

pub(crate) const PLUGIN: (StaticPlugin, StaticPlugin) = plugin("p", SingleProp("padding"));
pub(crate) const PLUGIN_X: (StaticPlugin, StaticPlugin) = plugin("px", SingleProp("padding-inline"));
pub(crate) const PLUGIN_Y: (StaticPlugin, StaticPlugin) = plugin("py", SingleProp("padding-block"));
pub(crate) const PLUGIN_START: (StaticPlugin, StaticPlugin) = plugin("ps", SingleProp("padding-inline-start"));
pub(crate) const PLUGIN_END: (StaticPlugin, StaticPlugin) = plugin("pe", SingleProp("padding-inline-end"));
pub(crate) const PLUGIN_TOP: (StaticPlugin, StaticPlugin) = plugin("pt", SingleProp("padding-top"));
pub(crate) const PLUGIN_BOTTOM: (StaticPlugin, StaticPlugin) = plugin("pb", SingleProp("padding-bottom"));
pub(crate) const PLUGIN_LEFT: (StaticPlugin, StaticPlugin) = plugin("pl", SingleProp("padding-left"));
pub(crate) const PLUGIN_RIGHT: (StaticPlugin, StaticPlugin) = plugin("pr", SingleProp("padding-right"));
