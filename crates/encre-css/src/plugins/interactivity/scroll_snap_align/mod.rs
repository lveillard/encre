#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("scroll-snap-align"),
    values: map! {
        "snap-start" => "start",
        "snap-end" => "end",
        "snap-center" => "center",
        "snap-align-none" => "none",
    },
});
