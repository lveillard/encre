#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "contrast",
    prop: SingleProp("--en-contrast"),
})
.divide_by(100.0)
.extra_lines(&[CSS_FILTER])
.template("contrast({})");
