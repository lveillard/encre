#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::{plugins::sizing::CSS_SIZE_VALUES_VERTICAL, prelude::build_plugin::*};

pub(crate) const PLUGIN_SPACING: StaticPlugin = Plugin::Spacing(Spacing {
    namespace: "min-h",
    prop: SingleProp("min-height"),
    has_auto: Some(true),
    has_full: Some(true),
    ..Spacing::default()
});

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("min-height"),
    values: CSS_SIZE_VALUES_VERTICAL,
    namespace: Some("min-h"),
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "min-h",
    prop: SingleProp("min-height"),
    ..Arbitrary::default()
});
