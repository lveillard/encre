#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "backdrop-hue-rotate",
    prop: SingleProp("--en-backdrop-hue-rotate"),
})
.has_negative()
.extra_lines(&CSS_BACKDROP_FILTER)
.template("hue-rotate({}deg)");

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "backdrop-hue-rotate",
    prop: SingleProp("--en-backdrop-hue-rotate"),
})
.extra_lines(&CSS_BACKDROP_FILTER)
.template("hue-rotate({})");
