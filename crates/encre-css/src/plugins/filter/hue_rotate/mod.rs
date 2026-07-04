#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::AnyNumber {
    prefix: "hue-rotate",
    prop: SingleProp("--en-hue-rotate"),
    has_empty: false,
    has_negative: true,
    divide_by: 1.0,
})
.extra_lines(&[CSS_FILTER])
.template("hue-rotate({}deg)");

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "hue-rotate",
    prop: SingleProp("--en-hue-rotate"),
})
.extra_lines(&[CSS_FILTER])
.template("hue-rotate({})");
