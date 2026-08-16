#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "sepia",
    prop: SingleProp("--en-sepia"),
    divide_by: Some(100.0),
    has_empty: Some(true),
    extra_lines: Some(&[CSS_FILTER]),
    template: Some("sepia({})"),
    ..Number::default()
});
