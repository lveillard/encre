#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Color {
    namespace: "ring-offset",
    prop: SingleProp("--en-ring-offset-color"),
})
.extra_lines(&["--en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);"]);

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "ring-offset",
    prop: SingleProp("--en-ring-offset-color"),
})
.hints(&[ArbitraryHint::Color])
.matcher(Color)
.extra_lines(&["--en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);"]);
