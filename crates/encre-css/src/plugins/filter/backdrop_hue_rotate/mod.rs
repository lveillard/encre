#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::Number {
    namespace: "backdrop-hue-rotate",
    prop: SingleProp("--en-backdrop-hue-rotate"),
    divide_by: 1.0,
})
.has_negative()
.extra_lines(&CSS_BACKDROP_FILTER)
.template("hue-rotate({}deg)");

pub(crate) const PLUGIN_ARBITRARY: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "backdrop-hue-rotate",
    prop: SingleProp("--en-backdrop-hue-rotate"),
})
.extra_lines(&CSS_BACKDROP_FILTER)
.template("hue-rotate({})");
