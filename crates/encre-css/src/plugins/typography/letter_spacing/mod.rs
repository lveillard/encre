#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("letter-spacing"),
    values: map! {
        "tracking-tighter" => "-0.05em",
        "tracking-tight" => "-0.025em",
        "tracking-normal" => "0",
        "tracking-wide" => "0.025em",
        "tracking-wider" => "0.05em",
        "tracking-widest" => "0.1em",
    },
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "tracking",
    prop: SingleProp("letter-spacing"),
});
