#![doc = include_str!("README.md")]
#![doc(alias("spacing", "space"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_X_1: Plugin = Plugin::new(PluginKind::ListCases {
    cases: phf_map! {
        "reverse" => &["--en-space-x-reverse: 1;"],
    },
}).extra_class(" > :not(:last-child)");

pub(crate) const PLUGIN_X_2: Plugin = Plugin::new(PluginKind::Spacing {
    prop: MultipleProps(&["margin-inline-start", "margin-inline-end"]),
    has_auto: false,
    has_full: false,
}).template_multiple(&[
    "calc({} * var(--en-space-x-reverse))",
    "calc({} * calc(1 - var(--en-space-x-reverse)))",
]).extra_lines(&[
    "--en-space-x-reverse: 0;"
]).extra_class(" > :not(:last-child)");

pub(crate) const PLUGIN_X_3: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: MultipleProps(&["margin-inline-start", "margin-inline-end"]),
    hints: &[],
    matcher: Or(&Length, &Percentage),
}).template_multiple(&[
    "calc({} * var(--en-space-x-reverse))",
    "calc({} * calc(1 - var(--en-space-x-reverse)))",
]).extra_lines(&[
    "--en-space-x-reverse: 0;"
]).extra_class(" > :not(:last-child)");

pub(crate) const PLUGIN_Y_1: Plugin = Plugin::new(PluginKind::ListCases {
    cases: phf_map! {
        "reverse" => &["--en-space-y-reverse: 1;"],
    },
}).extra_class(" > :not(:last-child)");

pub(crate) const PLUGIN_Y_2: Plugin = Plugin::new(PluginKind::Spacing {
    prop: MultipleProps(&["margin-block-start", "margin-block-end"]),
    has_auto: false,
    has_full: false,
}).template_multiple(&[
    "calc({} * var(--en-space-y-reverse))",
    "calc({} * calc(1 - var(--en-space-y-reverse)))",
]).extra_lines(&[
    "--en-space-y-reverse: 0;"
]).extra_class(" > :not(:last-child)");

pub(crate) const PLUGIN_Y_3: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: MultipleProps(&["margin-block-start", "margin-block-end"]),
    hints: &[],
    matcher: Or(&Length, &Percentage),
}).template_multiple(&[
    "calc({} * var(--en-space-y-reverse))",
    "calc({} * calc(1 - var(--en-space-y-reverse)))",
]).extra_lines(&[
    "--en-space-y-reverse: 0;"
]).extra_class(" > :not(:last-child)");
