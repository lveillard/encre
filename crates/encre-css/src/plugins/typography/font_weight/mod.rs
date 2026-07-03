#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("font-weight"),
    values: phf_map! {
        "thin" => "100",
        "extralight" => "200",
        "light" => "300",
        "normal" => "400",
        "medium" => "500",
        "semibold" => "600",
        "bold" => "700",
        "extrabold" => "800",
        "black" => "900",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: SingleProp("font-weight"),
})
.hints(&[PluginArbitraryHint::Number])
.matcher(OrMultiple(&[
    &CustomMultiple(&["normal", "bold", "lighter", "bolder"]),
    &Number,
    &Var,
]));
