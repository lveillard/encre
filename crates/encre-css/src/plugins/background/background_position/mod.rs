#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-position"),
    values: phf_map! {
        "bg-bottom" => "bottom",
        "bg-center" => "center",
        "bg-left" => "left",
        "bg-left-bottom" => "left bottom",
        "bg-left-top" => "left top",
        "bg-right" => "right",
        "bg-right-bottom" => "right bottom",
        "bg-right-top" => "right top",
        "bg-top" => "top",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "bg",
    prop: SingleProp("background-position"),
})
.hints(&[PluginArbitraryHint::Position])
.matcher(CommaSeparated(&SpaceSeparated(&Position)));
