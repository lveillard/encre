#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_NUMBER: StaticPlugin = Plugin::Number(Number {
    namespace: "grid-cols",
    prop: SingleProp("grid-template-columns"),
    template: Some("repeat({}, minmax(0, 1fr))"),
    ..Number::default()
});

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("grid-template-columns"),
    values: map! {
        "grid-cols-none" => "none",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "grid-cols",
    prop: SingleProp("grid-template-columns"),
    ..Arbitrary::default()
});
