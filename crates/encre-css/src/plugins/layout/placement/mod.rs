#![doc = include_str!("README.md")]
#![doc(alias("layout", "inset"))]
use crate::prelude::build_plugin::*;

const fn plugin(namespace: &'static str, prop: StaticPropertyName) -> (StaticPlugin, StaticPlugin) {
    (
        Plugin::new(PluginKind::Spacing { namespace, prop })
            .has_auto()
            .has_full(),
        Plugin::new(PluginKind::Arbitrary { namespace, prop }),
    )
}

pub(crate) const PLUGIN: (StaticPlugin, StaticPlugin) = plugin("inset", SingleProp("inset"));
pub(crate) const PLUGIN_X: (StaticPlugin, StaticPlugin) = plugin("inset-x", SingleProp("inset-inline"));
pub(crate) const PLUGIN_Y: (StaticPlugin, StaticPlugin) = plugin("inset-y", SingleProp("inset-block"));
pub(crate) const PLUGIN_START: (StaticPlugin, StaticPlugin) = plugin("start", SingleProp("inset-inline-start"));
pub(crate) const PLUGIN_END: (StaticPlugin, StaticPlugin) = plugin("end", SingleProp("inset-inline-end"));
pub(crate) const PLUGIN_TOP: (StaticPlugin, StaticPlugin) = plugin("top", SingleProp("top"));
pub(crate) const PLUGIN_BOTTOM: (StaticPlugin, StaticPlugin) = plugin("bottom", SingleProp("bottom"));
pub(crate) const PLUGIN_LEFT: (StaticPlugin, StaticPlugin) = plugin("left", SingleProp("left"));
pub(crate) const PLUGIN_RIGHT: (StaticPlugin, StaticPlugin) = plugin("right", SingleProp("right"));
