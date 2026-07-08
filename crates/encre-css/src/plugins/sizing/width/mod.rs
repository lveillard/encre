#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::{plugins::sizing::CSS_SIZE_VALUES_HORIZONTAL, prelude::build_plugin::*};

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Spacing {
    prefix: "w",
    prop: SingleProp("width"),
}).has_auto().has_full();

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("width"),
    values: CSS_SIZE_VALUES_HORIZONTAL,
}).list_prefix("w");

pub(crate) const PLUGIN_3: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "w",
    prop: SingleProp("width"),
});
