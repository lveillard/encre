#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-position"),
    values: map! {
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

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "bg",
    prop: SingleProp("background-position"),
})
.hints(&[ArbitraryHint::Position])
.matchers(&[Position], PluginArbitraryMatcherSeparation::Both);
