#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::{plugins::sizing::CSS_SIZE_VALUES_VERTICAL, prelude::build_plugin::*};

pub(crate) const PLUGIN_SPACING: StaticPlugin = Plugin::Spacing(Spacing {
    namespace: "max-h",
    prop: SingleProp("max-height"),
    has_auto: Some(true),
    has_full: Some(true),
    ..Spacing::default()
});

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("max-height"),
    values: CSS_SIZE_VALUES_VERTICAL,
    namespace: Some("max-h"),
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "max-h",
    prop: SingleProp("max-height"),
    ..Arbitrary::default()
});
