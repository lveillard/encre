#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("place-content"),
    values: map! {
        "place-content-start" => "start",
        "place-content-center" => "center",
        "place-content-end" => "end",
        "place-content-between" => "space-between",
        "place-content-around" => "space-around",
        "place-content-evenly" => "space-evenly",
    },
    ..ListValues::default()
});
