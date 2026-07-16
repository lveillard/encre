#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::{plugins::sizing::CSS_SIZE_VALUES_HORIZONTAL, prelude::build_plugin::*};

pub(crate) const PLUGIN_SPACING: StaticPlugin = Plugin::new(PluginKind::Spacing {
    namespace: "w",
    prop: SingleProp("width"),
}).has_auto().has_full();

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("width"),
    values: CSS_SIZE_VALUES_HORIZONTAL,
}).namespace("w");

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "w",
    prop: SingleProp("width"),
});
