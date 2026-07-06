#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_X_1: Plugin = Plugin::new(PluginKind::ListCases {
    cases: phf_map! {
        "divide-x-reverse" => &["--en-divide-x-reverse: 1;"],
    },
})
.extra_class(" > :not([hidden]) ~ :not([hidden])");

pub(crate) const PLUGIN_X_2: Plugin = Plugin::new(PluginKind::Number {
    prefix: "divide-x",
    prop: MultipleProps(&["border-inline-start-width", "border-inline-end-width"]),
    has_empty: true,
    has_negative: false,
    divide_by: 1.0,
})
.template_multiple(&[
    "calc({}px * var(--en-divide-x-reverse))",
    "calc({}px * calc(1 - var(--en-divide-x-reverse)))",
])
.extra_lines(&["--en-divide-x-reverse: 0;"])
.extra_class(" > :not([hidden]) ~ :not([hidden])");

pub(crate) const PLUGIN_X_3: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "divide-x",
    prop: MultipleProps(&["border-inline-start-width", "border-inline-end-width"]),
})
.hints(&[PluginArbitraryHint::Length])
.matcher(SpaceSeparated(&Or(&Length, &LineWidth)))
.template_multiple(&[
    "calc({} * var(--en-divide-x-reverse))",
    "calc({} * calc(1 - var(--en-divide-x-reverse)))",
])
.extra_lines(&["--en-divide-x-reverse: 0;"])
.extra_class(" > :not([hidden]) ~ :not([hidden])");

pub(crate) const PLUGIN_Y_1: Plugin = Plugin::new(PluginKind::ListCases {
    cases: phf_map! {
        "divide-y-reverse" => &["--en-divide-y-reverse: 1;"],
    },
})
.extra_class(" > :not([hidden]) ~ :not([hidden])");

pub(crate) const PLUGIN_Y_2: Plugin = Plugin::new(PluginKind::Number {
    prefix: "divide-y",
    prop: MultipleProps(&["border-block-start-width", "border-block-end-width"]),
    has_empty: true,
    has_negative: false,
    divide_by: 1.0,
})
.template_multiple(&[
    "calc({}px * var(--en-divide-y-reverse))",
    "calc({}px * calc(1 - var(--en-divide-y-reverse)))",
])
.extra_lines(&["--en-divide-y-reverse: 0;"])
.extra_class(" > :not([hidden]) ~ :not([hidden])");

pub(crate) const PLUGIN_Y_3: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "divide-y",
    prop: MultipleProps(&["border-block-start-width", "border-block-end-width"]),
})
.hints(&[PluginArbitraryHint::Length])
.matcher(SpaceSeparated(&Or(&Length, &LineWidth)))
.template_multiple(&[
    "calc({} * var(--en-divide-y-reverse))",
    "calc({} * calc(1 - var(--en-divide-y-reverse)))",
])
.extra_lines(&["--en-divide-y-reverse: 0;"])
.extra_class(" > :not([hidden]) ~ :not([hidden])");
