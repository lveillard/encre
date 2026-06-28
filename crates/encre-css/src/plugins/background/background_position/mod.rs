#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-position"),
    values: phf_map! {
        "bottom" => "bottom",
        "center" => "center",
        "left" => "left",
        "left-bottom" => "left bottom",
        "left-top" => "left top",
        "right" => "right",
        "right-bottom" => "right bottom",
        "right-top" => "right top",
        "top" => "top",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("background-position"),
    hints: &[PluginArbitraryHint::Position],
    matcher: CommaSeparated(&Position),
});
