#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-size"),
    values: phf_map! {
        "bg-auto" => "auto",
        "bg-cover" => "cover",
        "bg-contain" => "contain",
    },
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "bg",
    prop: SingleProp("background-size"),
})
.hints(&[ArbitraryHint::Length, ArbitraryHint::Percentage])
.matchers(
    &[
        Length,
        Percentage,
        Custom("auto"),
        Custom("cover"),
        Custom("contain"),
    ],
    PluginArbitraryMatcherModifier::Both,
);
