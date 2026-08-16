#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("scroll-snap-stop"),
    values: map! {
        "snap-normal" => "normal",
        "snap-always" => "always",
    },
    ..ListValues::default()
});
