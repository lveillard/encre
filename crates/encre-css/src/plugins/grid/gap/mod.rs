#![doc = include_str!("README.md")]
#![doc(alias("grid", "flexbox"))]
use crate::prelude::build_plugin::*;

const fn plugin(prefix: &'static str, prop: PropertyName) -> (Plugin, Plugin) {
    (
        Plugin::new(PluginKind::Spacing {
            prefix,
            prop,
            has_auto: false,
            has_full: false,
        }),
        Plugin::new(PluginKind::Arbitrary { prefix, prop }),
    )
}

pub(crate) const PLUGIN: (Plugin, Plugin) = plugin("gap", SingleProp("gap"));
pub(crate) const PLUGIN_X: (Plugin, Plugin) = plugin("gap-x", SingleProp("column-gap"));
pub(crate) const PLUGIN_Y: (Plugin, Plugin) = plugin("gap-y", SingleProp("row-gap"));
