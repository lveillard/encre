#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::{plugins::sizing::CSS_SIZE_VALUES_HORIZONTAL, prelude::build_plugin::*};

pub(crate) const PLUGIN_SPACING: StaticPlugin = Plugin::Spacing(Spacing {
    namespace: "min-w",
    prop: SingleProp("min-width"),
    has_auto: Some(true),
    has_full: Some(true),
    ..Spacing::default()
});

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("min-width"),
    values: CSS_SIZE_VALUES_HORIZONTAL,
    namespace: Some("min-w"),
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "min-w",
    prop: SingleProp("min-width"),
    ..Arbitrary::default()
});
