#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("scroll-behavior"),
    values: map! {
        "scroll-auto" => "auto",
        "scroll-smooth" => "smooth",
    },
    ..ListValues::default()
});
