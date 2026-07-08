#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::Number {
    prefix: "backdrop-saturate",
    prop: SingleProp("--en-backdrop-saturate"),
    divide_by: 100.0,
})
.extra_lines(&CSS_BACKDROP_FILTER)
.template("saturate({})");
