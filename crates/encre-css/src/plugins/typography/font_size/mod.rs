#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListCases {
    cases: phf_map! {
        "text-xs" => &["font-size: 0.75rem;", "line-height: 1rem;"],
        "text-sm" => &["font-size: 0.875rem;", "line-height: 1.25rem;"],
        "text-base" => &["font-size: 1rem;", "line-height: 1.5rem;"],
        "text-lg" => &["font-size: 1.125rem;", "line-height: 1.75rem;"],
        "text-xl" => &["font-size: 1.25rem;", "line-height: 1.75rem;"],
        "text-2xl" => &["font-size: 1.5rem;", "line-height: 2rem;"],
        "text-3xl" => &["font-size: 1.875rem;", "line-height: 2.25rem;"],
        "text-4xl" => &["font-size: 2.25rem;", "line-height: 2.5rem;"],
        "text-5xl" => &["font-size: 3rem;", "line-height: 1;"],
        "text-6xl" => &["font-size: 3.75rem;", "line-height: 1;"],
        "text-7xl" => &["font-size: 4.5rem;", "line-height: 1;"],
        "text-8xl" => &["font-size: 6rem;", "line-height: 1;"],
        "text-9xl" => &["font-size: 8rem;", "line-height: 1;"],
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "text",
    prop: SingleProp("font-size"),
})
.hints(&[
    ArbitraryHint::Length,
    ArbitraryHint::Percentage,
    ArbitraryHint::AbsoluteSize,
    ArbitraryHint::RelativeSize,
])
.matcher(OrMultiple(&[
    &Length,
    &Percentage,
    &AbsoluteSize,
    &RelativeSize,
]));
