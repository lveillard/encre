#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::{plugins::sizing::CSS_SIZE_VALUES_HORIZONTAL, prelude::build_plugin::*};

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Spacing {
    prefix: "min-w",
    prop: SingleProp("min-width"),
}).has_auto().has_full();

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("min-width"),
    values: CSS_SIZE_VALUES_HORIZONTAL,
}).list_prefix("min-w");

pub(crate) const PLUGIN_3: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "min-w",
    prop: SingleProp("min-width"),
});
