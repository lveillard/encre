#![doc = include_str!("README.md")]
#![doc(alias("background", "bg", "gradient"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_FROM_1: StaticPlugin = Plugin::new(PluginKind::Color {
    namespace: "from",
    prop: SingleProp("--en-gradient-from"),
})
.extra_lines(&[
    "--en-gradient-stops: var(--en-gradient-from), var(--en-gradient-to, transparent);",
]);

pub(crate) const PLUGIN_FROM_2: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "from",
    prop: SingleProp("--en-gradient-from"),
})
.extra_lines(&[
    "--en-gradient-stops: var(--en-gradient-from), var(--en-gradient-to, transparent);",
]);

pub(crate) const PLUGIN_VIA_1: StaticPlugin = Plugin::new(PluginKind::Color {
    namespace: "via",
    prop: SingleProp("--en-gradient-stops"),
})
.template("var(--en-gradient-from), {}, var(--en-gradient-to, transparent)");

pub(crate) const PLUGIN_VIA_2: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "via",
    prop: SingleProp("--en-gradient-stops"),
})
.template("var(--en-gradient-from), {}, var(--en-gradient-to, transparent)");

pub(crate) const PLUGIN_TO_1: StaticPlugin = Plugin::new(PluginKind::Color {
    namespace: "to",
    prop: SingleProp("--en-gradient-to"),
});

pub(crate) const PLUGIN_TO_2: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "to",
    prop: SingleProp("--en-gradient-to"),
});
