#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "backdrop-invert",
    prop: SingleProp("--en-backdrop-invert"),
    divide_by: 100.0,
})
.has_empty()
.extra_lines(&CSS_BACKDROP_FILTER)
.template("invert({})");
