#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("grid-row"),
    values: map! {
        "row-auto" => "auto",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "row",
    prop: SingleProp("grid-row"),
    ..Arbitrary::default()
});

pub(crate) const PLUGIN_SPAN_1: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("grid-row"),
    values: map! {
        "row-span-full" => "1 / -1",
    },
    ..ListValues::default()
});
pub(crate) const PLUGIN_SPAN_2: StaticPlugin = Plugin::Number(Number {
    namespace: "row-span",
    prop: SingleProp("grid-row"),
    template: Some(SingleProp("span {} / span {}")),
    ..Number::default()
});

pub(crate) const PLUGIN_START_1: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("grid-row-start"),
    values: map! {
        "row-start-auto" => "auto",
    },
    ..ListValues::default()
});
pub(crate) const PLUGIN_START_2: StaticPlugin = Plugin::Number(Number {
    namespace: "row-start",
    prop: SingleProp("grid-row-start"),
    ..Number::default()
});

pub(crate) const PLUGIN_END_1: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("grid-row-end"),
    values: map! {
        "row-end-auto" => "auto",
    },
    ..ListValues::default()
});
pub(crate) const PLUGIN_END_2: StaticPlugin = Plugin::Number(Number {
    namespace: "row-end",
    prop: SingleProp("grid-row-end"),
    ..Number::default()
});
