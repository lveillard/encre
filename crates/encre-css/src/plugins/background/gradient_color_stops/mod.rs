#![doc = include_str!("README.md")]
#![doc(alias("background", "bg", "gradient"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_FROM_1: StaticPlugin = Plugin::Color(Color {
    namespace: "from",
    prop: SingleProp("--en-gradient-from"),
    extra_rule_css: Some(&[
        "--en-gradient-stops: var(--en-gradient-from), var(--en-gradient-to, transparent);",
    ]),
    ..Color::default()
});

pub(crate) const PLUGIN_FROM_2: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "from",
    prop: SingleProp("--en-gradient-from"),
    extra_rule_css: Some(&[
        "--en-gradient-stops: var(--en-gradient-from), var(--en-gradient-to, transparent);",
    ]),
    ..Arbitrary::default()
});

pub(crate) const PLUGIN_VIA_1: StaticPlugin = Plugin::Color(Color {
    namespace: "via",
    prop: SingleProp("--en-gradient-stops"),
    template: Some(SingleProp("var(--en-gradient-from), {}, var(--en-gradient-to, transparent)")),
    ..Color::default()
});

pub(crate) const PLUGIN_VIA_2: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "via",
    prop: SingleProp("--en-gradient-stops"),
    template: Some(SingleProp("var(--en-gradient-from), {}, var(--en-gradient-to, transparent)")),
    ..Arbitrary::default()
});

pub(crate) const PLUGIN_TO_1: StaticPlugin = Plugin::Color(Color {
    namespace: "to",
    prop: SingleProp("--en-gradient-to"),
    ..Color::default()
});

pub(crate) const PLUGIN_TO_2: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "to",
    prop: SingleProp("--en-gradient-to"),
    ..Arbitrary::default()
});
