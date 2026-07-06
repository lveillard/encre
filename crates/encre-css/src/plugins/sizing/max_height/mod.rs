#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::{plugins::sizing::CSS_SIZE_VALUES_VERTICAL, prelude::build_plugin::*};

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Spacing {
    prefix: "max-h",
    prop: SingleProp("max-height"),
    has_auto: true,
    has_full: true,
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("max-height"),
    values: CSS_SIZE_VALUES_VERTICAL,
}).list_prefix("max-h");

pub(crate) const PLUGIN_3: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "max-h",
    prop: SingleProp("max-height"),
});
