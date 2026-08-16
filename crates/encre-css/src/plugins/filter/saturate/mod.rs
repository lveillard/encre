#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "saturate",
    prop: SingleProp("--en-saturate"),
    divide_by: Some(100.0),
    extra_lines: Some(&[CSS_FILTER]),
    template: Some("saturate({})"),
    ..Number::default()
});
