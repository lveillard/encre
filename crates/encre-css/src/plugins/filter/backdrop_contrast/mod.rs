#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "backdrop-contrast",
    prop: SingleProp("--en-backdrop-contrast"),
})
.divide_by(100.0)
.extra_lines(&CSS_BACKDROP_FILTER)
.template("contrast({})");
