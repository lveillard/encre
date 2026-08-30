#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::{plugins::sizing::CSS_SIZE_VALUES_HORIZONTAL, prelude::build_plugin::*};

pub(crate) const PLUGIN_SPACING: StaticPlugin = Plugin::Spacing(Spacing {
    namespace: "w",
    prop: SingleProp("width"),
    has_auto: Some(true),
    has_full: Some(true),
    ..Spacing::default()
});

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("width"),
    values: CSS_SIZE_VALUES_HORIZONTAL,
    namespace: Some("w"),
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "w",
    prop: SingleProp("width"),
    ..Arbitrary::default()
});
