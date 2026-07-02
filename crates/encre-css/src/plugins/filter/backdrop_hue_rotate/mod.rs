#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::AnyNumber {
    prop: SingleProp("--en-backdrop-hue-rotate"),
    has_empty: false,
    has_negative: true,
    divide_by: 1.0,
})
.extra_lines(&CSS_BACKDROP_FILTER)
.template("hue-rotate({}deg)");

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: SingleProp("--en-backdrop-hue-rotate"),
    hints: &[],
    matcher: Angle,
})
.extra_lines(&CSS_BACKDROP_FILTER)
.template("hue-rotate({})");
