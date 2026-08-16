#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("flex"),
    values: map! {
        "flex-1" => "1 1 0%",
        "flex-auto" => "1 1 auto",
        "flex-initial" => "0 1 auto",
        "flex-none" => "none",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "flex",
    prop: SingleProp("flex"),
    ..Arbitrary::default()
});
