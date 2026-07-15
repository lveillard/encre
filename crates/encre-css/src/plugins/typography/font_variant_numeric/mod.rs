#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListCases {
    cases: phf_map! {
        "normal-nums" => &[
            "font-variant-numeric: normal;",
        ],
        "ordinal" => &[
            "--en-ordinal: ordinal;",
            "font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);",
        ],
        "slashed-zero" => &[
            "--en-slashed-zero: slashed-zero;",
            "font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);",
        ],
        "lining-nums" => &[
            "--en-numeric-figure: lining-nums;",
            "font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);",
        ],
        "oldstyle-nums" => &[
            "--en-numeric-figure: oldstyle-nums;",
            "font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);",
        ],
        "proportional-nums" => &[
            "--en-numeric-spacing: proportional-nums;",
            "font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);",
        ],
        "tabular-nums" => &[
            "--en-numeric-spacing: tabular-nums;",
            "font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);",
        ],
        "diagonal-fractions" => &[
            "--en-numeric-fraction: diagonal-fractions;",
            "font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);",
        ],
        "stacked-fractions" => &[
            "--en-numeric-fraction: stacked-fractions;",
            "font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);",
        ],
    },
});
