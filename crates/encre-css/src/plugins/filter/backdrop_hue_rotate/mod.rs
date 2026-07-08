#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Number {
    prefix: "backdrop-hue-rotate",
    prop: SingleProp("--en-backdrop-hue-rotate"),
    divide_by: 1.0,
})
.has_negative()
.extra_lines(&CSS_BACKDROP_FILTER)
.template("hue-rotate({}deg)");

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "backdrop-hue-rotate",
    prop: SingleProp("--en-backdrop-hue-rotate"),
})
.extra_lines(&CSS_BACKDROP_FILTER)
.template("hue-rotate({})");
