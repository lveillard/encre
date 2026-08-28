#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("text-underline-offset"),
    values: map! {
        "underline-offset-auto" => "auto",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_NUMBER: StaticPlugin = Plugin::Number(Number {
    namespace: "underline-offset",
    prop: SingleProp("text-underline-offset"),
    template: Some(SingleProp("{}px")),
    ..Number::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "underline-offset",
    prop: SingleProp("text-underline-offset"),
    ..Arbitrary::default()
});
