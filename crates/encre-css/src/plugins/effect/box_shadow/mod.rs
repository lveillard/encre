#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::prelude::build_plugin::*;

// TODO: migrate to ListValues when extra_line will be more general
pub(crate) const PLUGIN_1: Plugin = Plugin::ListCases {
    cases: phf_map! {
        "2xs" => &[
            "--en-shadow: 0 1px var(--en-shadow-color, rgb(0 0 0 / 0.05));",
            "box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);",
        ],
        "xs" => &[
            "--en-shadow: 0 1px 2px 0 var(--en-shadow-color, rgb(0 0 0 / 0.05));",
            "box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);",
        ],
        "sm" => &[
            "--en-shadow: 0 1px 3px 0 var(--en-shadow-color, rgb(0 0 0 / 0.1)), 0 1px 2px -1px var(--en-shadow-color, rgb(0 0 0 / 0.1));",
            "box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);",
        ],
        "md" => &[
            "--en-shadow: 0 4px 6px -1px var(--en-shadow-color, rgb(0 0 0 / 0.1)), 0 2px 4px -2px var(--en-shadow-color, rgb(0 0 0 / 0.1));",
            "box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);",
        ],
        "lg" => &[
            "--en-shadow: 0 10px 15px -3px var(--en-shadow-color, rgb(0 0 0 / 0.1)), 0 4px 6px -4px var(--en-shadow-color, rgb(0 0 0 / 0.1));",
            "box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);",
        ],
        "xl" => &[
            "--en-shadow: 0 20px 25px -5px var(--en-shadow-color, rgb(0 0 0 / 0.1)), 0 8px 10px -6px var(--en-shadow-color, rgb(0 0 0 / 0.1));",
            "box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);",
        ],
        "2xl" => &[
            "--en-shadow: 0 25px 50px -12px var(--en-shadow-color, rgb(0 0 0 / 0.25));",
            "box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);",
        ],
        "none" => &[
            "--en-shadow: 0 0 #0000;",
            "box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);",
        ],
    },
};

pub(crate) const PLUGIN_2: Plugin = Plugin::ArbitraryShadow {
  prop: SingleProp("--en-shadow"),
  color_replacement: "var(--en-shadow-color, {})",
  extra_line: "box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);",
};

pub(crate) const PLUGIN_INSET_1: Plugin = Plugin::ListCases {
    cases: phf_map! {
        "2xs" => &[
            "--en-inset-shadow: inset 0 1px var(--en-inset-shadow-color, rgb(0 0 0 / 0.05));",
            "box-shadow: var(--en-inset-shadow), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow, 0 0 #0000);",
        ],
        "xs" => &[
            "--en-inset-shadow: inset 0 1px 1px var(--en-inset-shadow-color, rgb(0 0 0 / 0.05));",
            "box-shadow: var(--en-inset-shadow), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow, 0 0 #0000);",
        ],
        "sm" => &[
            "--en-inset-shadow: 0 2px 4px var(--en-inset-shadow-color, rgb(0 0 0 / 0.05));",
            "box-shadow: var(--en-inset-shadow), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow, 0 0 #0000);",
        ],
        "none" => &[
            "--en-inset-shadow: inset 0 0 #0000;",
            "box-shadow: var(--en-inset-shadow), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow, 0 0 #0000);",
        ],
    },
};

pub(crate) const PLUGIN_INSET_2: Plugin = Plugin::ArbitraryShadow {
  prop: SingleProp("--en-inset-shadow"),
  color_replacement: "var(--en-inset-shadow-color, {})",
  extra_line: "box-shadow: var(--en-inset-shadow), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow, 0 0 #0000);",
};
