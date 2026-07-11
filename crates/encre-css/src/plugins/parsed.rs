use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::selector::ArbitraryHint;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub(crate) enum ParsedPluginArbitraryMatcher {
    All,
    Url,
    Var,
    Shadow,
    AbsoluteSize,
    RelativeSize,
    LineWidth,
    LineStyle,
    ComputationalCssFunction,
    Color,
    Length,
    Number,
    Percentage,
    Time,
    Gradient,
    Position,
    Angle,
    Image,
    FontFamilyName,
    Custom(String),
    CustomMultiple(Vec<String>),

    Or(
        Box<ParsedPluginArbitraryMatcher>,
        Box<ParsedPluginArbitraryMatcher>,
    ),
    OrMultiple(Vec<ParsedPluginArbitraryMatcher>),
    CommaSeparated(Box<ParsedPluginArbitraryMatcher>),
    SpaceSeparated(Box<ParsedPluginArbitraryMatcher>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) enum ParsedPropertyName {
    SingleProp(String),
    MultipleProps(Vec<String>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) enum ParsedPluginKind {
    ListCases {
        cases: HashMap<String, Vec<String>>,
    },
    ListValues {
        prop: ParsedPropertyName,
        values: HashMap<String, String>,
    },

    Spacing {
        namespace: String,
        prop: ParsedPropertyName,
    },
    Color {
        namespace: String,
        prop: ParsedPropertyName,
    },
    Number {
        namespace: String,
        prop: ParsedPropertyName,
        divide_by: f32,
    },

    Arbitrary {
        namespace: String,
        prop: ParsedPropertyName,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct ParsedPlugin {
    pub(crate) kind: ParsedPluginKind,

    #[serde(default)]
    pub(crate) has_auto: bool,

    #[serde(default)]
    pub(crate) has_empty: bool,

    #[serde(default)]
    pub(crate) has_full: bool,

    #[serde(default)]
    pub(crate) has_negative: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) extra_lines: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) extra_css: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) extra_class: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) extra_slash: Option<(HashMap<String, String>, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) template_multiple: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) arbitrary_hints: Option<Vec<ArbitraryHint>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) arbitrary_matcher: Option<ParsedPluginArbitraryMatcher>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) arbitrary_shadow_color_replacement: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) list_namespace: Option<String>,
}

impl ParsedPlugin {
    pub(crate) const fn new(kind: ParsedPluginKind) -> Self {
        Self {
            kind,
            has_auto: false,
            has_empty: false,
            has_full: false,
            has_negative: false,
            extra_lines: None,
            extra_css: None,
            extra_class: None,
            extra_slash: None,
            template: None,
            template_multiple: None,
            arbitrary_hints: None,
            arbitrary_matcher: None,
            arbitrary_shadow_color_replacement: None,
            list_namespace: None,
        }
    }
}
