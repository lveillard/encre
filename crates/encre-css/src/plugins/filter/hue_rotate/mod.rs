#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "hue-rotate",
    prop: SingleProp("--en-hue-rotate"),
    has_negative: Some(true),
    extra_rule_css: Some(&[CSS_FILTER]),
    template: Some(SingleProp("hue-rotate({}deg)")),
    ..Number::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "hue-rotate",
    prop: SingleProp("--en-hue-rotate"),
    extra_rule_css: Some(&[CSS_FILTER]),
    template: Some(SingleProp("hue-rotate({})")),
    ..Arbitrary::default()
});
