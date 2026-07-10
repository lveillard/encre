#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("border-style"),
    values: phf_map! {
        "border-solid" => "solid",
        "border-dashed" => "dashed",
        "border-dotted" => "dotted",
        "border-double" => "double",
        "border-groove" => "groove",
        "border-ridge" => "ridge",
        "border-inset" => "inset",
        "border-outset" => "outset",
        "border-hidden" => "hidden",
        "border-none" => "none",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "border",
    prop: SingleProp("border-style"),
})
.hints(&[])
.matcher(SpaceSeparated(&LineStyle));
