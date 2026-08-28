#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_NUMBER: StaticPlugin = Plugin::Number(Number {
    namespace: "grid-rows",
    prop: SingleProp("grid-template-rows"),
    template: Some(SingleProp("repeat({}, minmax(0, 1fr))")),
    ..Number::default()
});

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("grid-template-rows"),
    values: map! {
        "grid-rows-none" => "none",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "grid-rows",
    prop: SingleProp("grid-template-rows"),
    ..Arbitrary::default()
});
