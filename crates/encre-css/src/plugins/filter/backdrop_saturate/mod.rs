#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "backdrop-saturate",
    prop: SingleProp("--en-backdrop-saturate"),
    divide_by: Some(100.0),
    extra_lines: Some(&CSS_BACKDROP_FILTER),
    template: Some("saturate({})"),
    ..Number::default()
});
