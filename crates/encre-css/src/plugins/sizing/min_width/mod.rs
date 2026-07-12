#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::{plugins::sizing::CSS_SIZE_VALUES_HORIZONTAL, prelude::build_plugin::*};

pub(crate) const PLUGIN_SPACING: Plugin = Plugin::new(PluginKind::Spacing {
    namespace: "min-w",
    prop: SingleProp("min-width"),
}).has_auto().has_full();

pub(crate) const PLUGIN_LIST: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("min-width"),
    values: CSS_SIZE_VALUES_HORIZONTAL,
}).list_namespace("min-w");

pub(crate) const PLUGIN_ARBITRARY: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "min-w",
    prop: SingleProp("min-width"),
});
