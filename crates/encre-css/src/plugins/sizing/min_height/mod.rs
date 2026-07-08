#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::{plugins::sizing::CSS_SIZE_VALUES_VERTICAL, prelude::build_plugin::*};

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Spacing {
    prefix: "min-h",
    prop: SingleProp("min-height"),
}).has_auto().has_full();

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("min-height"),
    values: CSS_SIZE_VALUES_VERTICAL,
}).list_prefix("min-h");

pub(crate) const PLUGIN_3: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "min-h",
    prop: SingleProp("min-height"),
});
