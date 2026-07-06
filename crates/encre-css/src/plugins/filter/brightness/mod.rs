#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::Number {
    prefix: "brightness",
    prop: SingleProp("--en-brightness"),
    has_empty: false,
    has_negative: false,
    divide_by: 100.0,
})
.extra_lines(&[CSS_FILTER])
.template("brightness({})");
