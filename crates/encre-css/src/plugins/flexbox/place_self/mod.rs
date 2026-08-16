#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("place-self"),
    values: map! {
        "place-self-auto" => "auto",
        "place-self-start" => "flex-start",
        "place-self-center" => "center",
        "place-self-end" => "flex-end",
        "place-self-stretch" => "stretch",
    },
    ..ListValues::default()
});
