#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("font-weight"),
    values: phf_map! {
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
});

pub(crate) const PLUGIN_ARBITRARY: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "font",
    prop: SingleProp("font-weight"),
})
.hints(&[ArbitraryHint::Number])
.matcher(OrMultiple(&[
    &CustomMultiple(&["normal", "bold", "lighter", "bolder"]),
    &Number,
    &Var,
]));
