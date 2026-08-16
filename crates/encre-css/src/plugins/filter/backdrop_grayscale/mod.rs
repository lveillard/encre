#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "backdrop-grayscale",
    prop: SingleProp("--en-backdrop-grayscale"),
    divide_by: Some(100.0),
    has_empty: Some(true),
    extra_lines: Some(&CSS_BACKDROP_FILTER),
    template: Some("grayscale({})"),
    ..Number::default()
});
