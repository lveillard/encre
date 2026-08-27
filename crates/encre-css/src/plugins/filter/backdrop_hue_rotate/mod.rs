#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "backdrop-hue-rotate",
    prop: SingleProp("--en-backdrop-hue-rotate"),
    has_negative: Some(true),
    extra_rule_css: Some(&CSS_BACKDROP_FILTER),
    template: Some("hue-rotate({}deg)"),
    ..Number::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "backdrop-hue-rotate",
    prop: SingleProp("--en-backdrop-hue-rotate"),
    extra_rule_css: Some(&CSS_BACKDROP_FILTER),
    template: Some("hue-rotate({})"),
    ..Arbitrary::default()
});
