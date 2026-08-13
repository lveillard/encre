#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "hue-rotate",
    prop: SingleProp("--en-hue-rotate"),
})
.has_negative()
.extra_lines(&[CSS_FILTER])
.template("hue-rotate({}deg)");

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "hue-rotate",
    prop: SingleProp("--en-hue-rotate"),
})
.extra_lines(&[CSS_FILTER])
.template("hue-rotate({})");
