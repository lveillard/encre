#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::{plugins::PluginArbitraryMatcher::*, prelude::build_plugin::*};

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("--en-shadow"),
    values: phf_map! {
        "shadow-2xs" => "0 1px var(--en-shadow-color, rgb(0 0 0 / 0.05))",
        "shadow-xs" => "0 1px 2px 0 var(--en-shadow-color, rgb(0 0 0 / 0.05))",
        "shadow-sm" => "0 1px 3px 0 var(--en-shadow-color, rgb(0 0 0 / 0.1)), 0 1px 2px -1px var(--en-shadow-color, rgb(0 0 0 / 0.1))",
        "shadow-md" => "0 4px 6px -1px var(--en-shadow-color, rgb(0 0 0 / 0.1)), 0 2px 4px -2px var(--en-shadow-color, rgb(0 0 0 / 0.1))",
        "shadow-lg" => "0 10px 15px -3px var(--en-shadow-color, rgb(0 0 0 / 0.1)), 0 4px 6px -4px var(--en-shadow-color, rgb(0 0 0 / 0.1))",
        "shadow-xl" => "0 20px 25px -5px var(--en-shadow-color, rgb(0 0 0 / 0.1)), 0 8px 10px -6px var(--en-shadow-color, rgb(0 0 0 / 0.1))",
        "shadow-2xl" => "0 25px 50px -12px var(--en-shadow-color, rgb(0 0 0 / 0.25))",
        "shadow-none" => "0 0 #0000",
    },
}).extra_lines(&["box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);"]);

pub(crate) const PLUGIN_ARBITRARY: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "shadow",
    prop: SingleProp("--en-shadow"),
})
.extra_lines(&["box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);"])
.matchers(&[Shadow], PluginArbitraryMatcherModifier::None)
.shadow_color_replacement("var(--en-shadow-color, {})");

pub(crate) const PLUGIN_INSET_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("--en-inset-shadow"),
    values: phf_map! {
        "inset-shadow-2xs" => "inset 0 1px var(--en-inset-shadow-color, rgb(0 0 0 / 0.05))",
        "inset-shadow-xs" => "inset 0 1px 1px var(--en-inset-shadow-color, rgb(0 0 0 / 0.05))",
        "inset-shadow-sm" => "0 2px 4px var(--en-inset-shadow-color, rgb(0 0 0 / 0.05))",
        "inset-shadow-none" => "inset 0 0 #0000",
    },
}).extra_lines(&["box-shadow: var(--en-inset-shadow), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow, 0 0 #0000);"]);

pub(crate) const PLUGIN_INSET_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "inset-shadow",
    prop: SingleProp("--en-inset-shadow"),
})
.extra_lines(&["box-shadow: var(--en-inset-shadow), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow, 0 0 #0000);"])
.hints(&[ArbitraryHint::Shadow])
.matchers(&[Shadow], PluginArbitraryMatcherModifier::None)
.shadow_color_replacement("var(--en-inset-shadow-color, {})");
