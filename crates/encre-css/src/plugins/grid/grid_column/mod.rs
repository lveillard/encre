#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("grid-column"),
    values: map! {
        "col-auto" => "auto"
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "col",
    prop: SingleProp("grid-column"),
    ..Arbitrary::default()
});

pub(crate) const PLUGIN_SPAN_1: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("grid-column"),
    values: map! {
        "col-span-full" => "1 / -1",
    },
    ..ListValues::default()
});
pub(crate) const PLUGIN_SPAN_2: StaticPlugin = Plugin::Number(Number {
    namespace: "col-span",
    prop: SingleProp("grid-column"),
    template: Some(SingleProp("span {} / span {}")),
    ..Number::default()
});

pub(crate) const PLUGIN_START_1: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("grid-column-start"),
    values: map! {
        "col-start-auto" => "auto",
    },
    ..ListValues::default()
});
pub(crate) const PLUGIN_START_2: StaticPlugin = Plugin::Number(Number {
    namespace: "col-start",
    prop: SingleProp("grid-column-start"),
    ..Number::default()
});

pub(crate) const PLUGIN_END_1: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("grid-column-end"),
    values: map! {
        "col-end-auto" => "auto",
    },
    ..ListValues::default()
});
pub(crate) const PLUGIN_END_2: StaticPlugin = Plugin::Number(Number {
    namespace: "col-end",
    prop: SingleProp("grid-column-end"),
    ..Number::default()
});
