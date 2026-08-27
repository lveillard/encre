#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "contrast",
    prop: SingleProp("--en-contrast"),
    divide_by: Some(100.0),
    extra_rule_css: Some(&[CSS_FILTER]),
    template: Some("contrast({})"),
    ..Number::default()
});
