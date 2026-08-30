#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("font-weight"),
    values: map! {
        "font-thin" => "100",
        "font-extralight" => "200",
        "font-light" => "300",
        "font-normal" => "400",
        "font-medium" => "500",
        "font-semibold" => "600",
        "font-bold" => "700",
        "font-extrabold" => "800",
        "font-black" => "900",
    },
    ..ListValues::default()
});

// This plugin is the default when encountering a `font-` utility class with an arbitrary
// value, so it does not need disambiguation
pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "font",
    prop: SingleProp("font-weight"),
    ..Arbitrary::default()
});
