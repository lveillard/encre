#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::AnyNumber {
    prefix: "ring-offset",
    prop: SingleProp("--en-ring-offset-width"),
    has_empty: false,
    has_negative: false,
    divide_by: 1.0,
})
.extra_lines(&["--en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);"])
.template("{}px");

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "ring-offset",
    prop: SingleProp("--en-ring-offset-width"),
})
.hints(&[PluginArbitraryHint::Length])
.matcher(Length)
.extra_lines(&["--en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);"]);
