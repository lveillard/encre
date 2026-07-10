#![doc = include_str!("README.md")]
#![doc(alias("grid", "flexbox"))]
use crate::prelude::build_plugin::*;

const fn plugin(namespace: &'static str, prop: PropertyName) -> (Plugin, Plugin) {
    (
        Plugin::new(PluginKind::Spacing {
            namespace,
            prop,
        }),
        Plugin::new(PluginKind::Arbitrary { namespace, prop }),
    )
}

pub(crate) const PLUGIN: (Plugin, Plugin) = plugin("gap", SingleProp("gap"));
pub(crate) const PLUGIN_X: (Plugin, Plugin) = plugin("gap-x", SingleProp("column-gap"));
pub(crate) const PLUGIN_Y: (Plugin, Plugin) = plugin("gap-y", SingleProp("row-gap"));
