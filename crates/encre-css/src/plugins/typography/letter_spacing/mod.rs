#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("letter-spacing"),
    values: phf_map! {
        "tracking-tighter" => "-0.05em",
        "tracking-tight" => "-0.025em",
        "tracking-normal" => "0",
        "tracking-wide" => "0.025em",
        "tracking-wider" => "0.05em",
        "tracking-widest" => "0.1em",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "tracking",
    prop: SingleProp("letter-spacing"),
});
