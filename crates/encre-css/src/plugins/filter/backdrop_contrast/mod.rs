#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "backdrop-contrast",
    prop: SingleProp("--en-backdrop-contrast"),
    divide_by: Some(100.0),
    extra_lines: Some(&CSS_BACKDROP_FILTER),
    template: Some("contrast({})"),
    ..Number::default()
});
