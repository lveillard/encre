#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::{plugins::sizing::CSS_SIZE_VALUES_VERTICAL, prelude::build_plugin::*};

pub(crate) const PLUGIN_SPACING: StaticPlugin = Plugin::new(PluginKind::Spacing {
    namespace: "max-h",
    prop: SingleProp("max-height"),
}).has_auto().has_full();

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("max-height"),
    values: CSS_SIZE_VALUES_VERTICAL,
}).namespace("max-h");

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "max-h",
    prop: SingleProp("max-height"),
});
