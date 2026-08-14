#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_NUMBER: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "decoration",
    prop: SingleProp("text-decoration-thickness"),
})
.template("{}px");

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("text-decoration-thickness"),
    values: phf_map! {
        "decoration-auto" => "auto",
        "decoration-from-font" => "from-font",
    },
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "decoration",
    prop: SingleProp("text-decoration-thickness"),
})
.hints(&[ArbitraryHint::Length, ArbitraryHint::Percentage])
.matchers(&[
    Length,
    Percentage,
    CustomMultiple(&["auto", "from-font"]),
], PluginArbitraryMatcherSeparation::None);
