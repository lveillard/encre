#![doc = include_str!("README.md")]
#![doc(alias("grid", "flexbox"))]
use crate::prelude::build_plugin::*;

const fn plugin(namespace: &'static str, prop: StaticPropertyName) -> (StaticPlugin, StaticPlugin) {
    (
        Plugin::Spacing(Spacing {
            namespace,
            prop,
            ..Spacing::default()
        }),
        Plugin::Arbitrary(Arbitrary {
            namespace,
            prop,
            ..Arbitrary::default()
        }),
    )
}

pub(crate) const PLUGIN: (StaticPlugin, StaticPlugin) = plugin("gap", SingleProp("gap"));
pub(crate) const PLUGIN_X: (StaticPlugin, StaticPlugin) = plugin("gap-x", SingleProp("column-gap"));
pub(crate) const PLUGIN_Y: (StaticPlugin, StaticPlugin) = plugin("gap-y", SingleProp("row-gap"));
