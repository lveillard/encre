#![doc = include_str!("README.md")]
#![doc(alias("spacing", "space"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_X_1: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("--en-space-x-reverse"),
    values: map! {
        "space-x-reverse" => "1",
    },
}).extra_class(" > :not(:last-child)");

pub(crate) const PLUGIN_X_2: StaticPlugin = Plugin::new(PluginKind::Spacing {
    namespace: "space-x",
    prop: MultipleProps(&["margin-inline-start", "margin-inline-end"]),
}).template_multiple(&[
    "calc({} * var(--en-space-x-reverse))",
    "calc({} * calc(1 - var(--en-space-x-reverse)))",
]).extra_lines(&[
    "--en-space-x-reverse: 0;"
]).extra_class(" > :not(:last-child)");

pub(crate) const PLUGIN_X_3: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "space-x",
    prop: MultipleProps(&["margin-inline-start", "margin-inline-end"]),
}).template_multiple(&[
    "calc({} * var(--en-space-x-reverse))",
    "calc({} * calc(1 - var(--en-space-x-reverse)))",
]).extra_lines(&[
    "--en-space-x-reverse: 0;"
]).extra_class(" > :not(:last-child)");

pub(crate) const PLUGIN_Y_1: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("--en-space-y-reverse"),
    values: map! {
        "space-y-reverse" => "1",
    },
}).extra_class(" > :not(:last-child)");

pub(crate) const PLUGIN_Y_2: StaticPlugin = Plugin::new(PluginKind::Spacing {
    namespace: "space-y",
    prop: MultipleProps(&["margin-block-start", "margin-block-end"]),
}).template_multiple(&[
    "calc({} * var(--en-space-y-reverse))",
    "calc({} * calc(1 - var(--en-space-y-reverse)))",
]).extra_lines(&[
    "--en-space-y-reverse: 0;"
]).extra_class(" > :not(:last-child)");

pub(crate) const PLUGIN_Y_3: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "space-y",
    prop: MultipleProps(&["margin-block-start", "margin-block-end"]),
}).template_multiple(&[
    "calc({} * var(--en-space-y-reverse))",
    "calc({} * calc(1 - var(--en-space-y-reverse)))",
]).extra_lines(&[
    "--en-space-y-reverse: 0;"
]).extra_class(" > :not(:last-child)");
