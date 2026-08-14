#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "ring",
    prop: SingleProp("--en-ring-shadow"),
}).has_empty().extra_lines(&[
    "box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow), var(--en-shadow, 0 0 #0000);",
]).template("0 0 0 calc({}px + var(--en-ring-offset-width)) var(--en-ring-color)");

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "ring",
    prop: SingleProp("--en-ring-shadow"),
})
.hints(&[ArbitraryHint::Length])
.matchers(&[Length], PluginArbitraryMatcherSeparation::None)
.extra_lines(&[
    "box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow), var(--en-shadow, 0 0 #0000);",
]).template("var(--en-ring-inset) 0 0 0 calc({} + var(--en-ring-offset-width)) var(--en-ring-color)");

pub(crate) const PLUGIN_INSET_1: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "inset-ring",
    prop: SingleProp("--en-inset-ring-shadow"),
}).has_empty().extra_lines(&[
    "box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow, 0 0 #0000);"
]).template("inset 0 0 0 calc({}px + var(--en-ring-offset-width)) var(--en-ring-color)");

pub(crate) const PLUGIN_INSET_2: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "inset-ring",
    prop: SingleProp("--en-inset-ring-shadow"),
})
.hints(&[ArbitraryHint::Length])
.matchers(&[Length], PluginArbitraryMatcherSeparation::None)
.extra_lines(&[
    "box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow, 0 0 #0000);"
]).template("inset 0 0 0 calc({value} + var(--en-ring-offset-width)) var(--en-ring-color)");
