#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::Number {
    prefix: "invert",
    prop: SingleProp("--en-invert"),
    divide_by: 100.0,
})
.has_empty()
.extra_lines(&[CSS_FILTER])
.template("invert({})");
