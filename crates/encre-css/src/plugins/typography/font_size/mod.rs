#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::ListCases {
    cases: phf_map! {
        "xs" => &["font-size: 0.75rem;", "line-height: 1rem;"],
        "sm" => &["font-size: 0.875rem;", "line-height: 1.25rem;"],
        "base" => &["font-size: 1rem;", "line-height: 1.5rem;"],
        "lg" => &["font-size: 1.125rem;", "line-height: 1.75rem;"],
        "xl" => &["font-size: 1.25rem;", "line-height: 1.75rem;"],
        "2xl" => &["font-size: 1.5rem;", "line-height: 2rem;"],
        "3xl" => &["font-size: 1.875rem;", "line-height: 2.25rem;"],
        "4xl" => &["font-size: 2.25rem;", "line-height: 2.5rem;"],
        "5xl" => &["font-size: 3rem;", "line-height: 1;"],
        "6xl" => &["font-size: 3.75rem;", "line-height: 1;"],
        "7xl" => &["font-size: 4.5rem;", "line-height: 1;"],
        "8xl" => &["font-size: 6rem;", "line-height: 1;"],
        "9xl" => &["font-size: 8rem;", "line-height: 1;"],
    },
};

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("font-size"),
    hints: &[
        PluginArbitraryHint::Length,
        PluginArbitraryHint::Percentage,
        PluginArbitraryHint::AbsoluteSize,
        PluginArbitraryHint::RelativeSize,
    ],
    matcher: OrMultiple(&[&Length, &Percentage, &AbsoluteSize, &RelativeSize]),
};
