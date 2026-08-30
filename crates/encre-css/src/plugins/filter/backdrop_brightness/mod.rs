#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "backdrop-brightness",
    prop: SingleProp("--en-backdrop-brightness"),
    divide_by: Some(100.0),
    template: Some(SingleProp("brightness({})")),
    extra_rule_css: Some(&CSS_BACKDROP_FILTER),
    ..Number::default()
});
