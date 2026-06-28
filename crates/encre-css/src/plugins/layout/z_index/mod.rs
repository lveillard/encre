#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::SamePropValues {
    prop: SingleProp("z-index"),
    values: &["auto"],
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::AnyNumber {
    prop: SingleProp("z-index"),
    has_empty: false,
    has_negative: true,
    divide_by: 1.0,
});
