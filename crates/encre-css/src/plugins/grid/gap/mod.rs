#![doc = include_str!("README.md")]
#![doc(alias("grid", "flexbox"))]
use crate::prelude::build_plugin::*;

const fn plugin(namespace: &'static str, prop: StaticPropertyName) -> (StaticPlugin, StaticPlugin) {
    (
        Plugin::new(PluginKind::Spacing {
            namespace,
            prop,
        }),
        Plugin::new(PluginKind::Arbitrary { namespace, prop }),
    )
}

pub(crate) const PLUGIN: (StaticPlugin, StaticPlugin) = plugin("gap", SingleProp("gap"));
pub(crate) const PLUGIN_X: (StaticPlugin, StaticPlugin) = plugin("gap-x", SingleProp("column-gap"));
pub(crate) const PLUGIN_Y: (StaticPlugin, StaticPlugin) = plugin("gap-y", SingleProp("row-gap"));
