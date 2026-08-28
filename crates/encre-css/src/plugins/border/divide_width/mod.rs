#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_X_1: StaticPlugin = Plugin::ListProperties(ListProperties {
    props: map! {
        "divide-x-reverse" => &["--en-divide-x-reverse: 1;"],
    },
    extra_class: Some(" > :not([hidden]) ~ :not([hidden])"),
    ..ListProperties::default()
});

pub(crate) const PLUGIN_X_2: StaticPlugin = Plugin::Number(Number {
    namespace: "divide-x",
    prop: MultipleProps(&["border-inline-start-width", "border-inline-end-width"]),
    has_empty: Some(true),
    extra_class: Some(" > :not([hidden]) ~ :not([hidden])"),
    template: Some(MultipleProps(&[
        "calc({}px * var(--en-divide-x-reverse))",
        "calc({}px * calc(1 - var(--en-divide-x-reverse)))",
    ])),
    extra_rule_css: Some(&["--en-divide-x-reverse: 0;"]),
    ..Number::default()
});

pub(crate) const PLUGIN_X_3: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "divide-x",
    prop: MultipleProps(&["border-inline-start-width", "border-inline-end-width"]),
    extra_class: Some(" > :not([hidden]) ~ :not([hidden])"),
    hints: Some(&[ArbitraryHint::Length]),
    matchers: Some((
        &[
            PluginArbitraryMatcher::Length,
            PluginArbitraryMatcher::LineWidth,
        ],
        PluginArbitraryMatcherSeparation::Space,
    )),
    template: Some(MultipleProps(&[
        "calc({} * var(--en-divide-x-reverse))",
        "calc({} * calc(1 - var(--en-divide-x-reverse)))",
    ])),
    extra_rule_css: Some(&["--en-divide-x-reverse: 0;"]),
    ..Arbitrary::default()
});

pub(crate) const PLUGIN_Y_1: StaticPlugin = Plugin::ListProperties(ListProperties {
    props: map! {
        "divide-y-reverse" => &["--en-divide-y-reverse: 1;"],
    },
    extra_class: Some(" > :not([hidden]) ~ :not([hidden])"),
    ..ListProperties::default()
});

pub(crate) const PLUGIN_Y_2: StaticPlugin = Plugin::Number(Number {
    namespace: "divide-y",
    prop: MultipleProps(&["border-block-start-width", "border-block-end-width"]),
    has_empty: Some(true),
    extra_class: Some(" > :not([hidden]) ~ :not([hidden])"),
    template: Some(MultipleProps(&[
        "calc({}px * var(--en-divide-y-reverse))",
        "calc({}px * calc(1 - var(--en-divide-y-reverse)))",
    ])),
    extra_rule_css: Some(&["--en-divide-y-reverse: 0;"]),
    ..Number::default()
});

pub(crate) const PLUGIN_Y_3: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "divide-y",
    prop: MultipleProps(&["border-block-start-width", "border-block-end-width"]),
    extra_class: Some(" > :not([hidden]) ~ :not([hidden])"),
    hints: Some(&[ArbitraryHint::Length]),
    matchers: Some((
        &[
            PluginArbitraryMatcher::Length,
            PluginArbitraryMatcher::LineWidth,
        ],
        PluginArbitraryMatcherSeparation::Space,
    )),
    template: Some(MultipleProps(&[
        "calc({} * var(--en-divide-y-reverse))",
        "calc({} * calc(1 - var(--en-divide-y-reverse)))",
    ])),
    extra_rule_css: Some(&["--en-divide-y-reverse: 0;"]),
    ..Arbitrary::default()
});
