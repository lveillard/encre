#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::ListValues {
    prop: SingleProp("letter-spacing"),
    values: phf_map! {
        "tighter" => "-0.05em",
        "tight" => "-0.025em",
        "normal" => "0",
        "wide" => "0.025em",
        "wider" => "0.05em",
        "widest" => "0.1em",
    },
};

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("letter-spacing"),
    hints: &[],
    matcher: Or(&Custom("nprmal"), &Length),
};
