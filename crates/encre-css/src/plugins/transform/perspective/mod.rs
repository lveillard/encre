#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("perspective"),
    values: map! {
        "perspective-dramatic" => "100px",
        "perspective-near" => "300px",
        "perspective-normal" => "500px",
        "perspective-midrange" => "800px",
        "perspective-distant" => "1200px",
        "perspective-none" => "none",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "perspective",
    prop: SingleProp("perspective"),
    ..Arbitrary::default()
});
