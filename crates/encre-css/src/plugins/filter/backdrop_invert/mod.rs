#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::AnyNumber {
    prop: SingleProp("--en-backdrop-invert"),
    has_empty: true,
    has_negative: false,
    divide_by: 100.0,
})
.extra_lines(&CSS_BACKDROP_FILTER)
.template("invert({})");
