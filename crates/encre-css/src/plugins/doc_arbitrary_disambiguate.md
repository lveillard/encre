Define a CSS value type which is used to disambiguate the use of the arbitrary plugin.

When two [`Arbitrary`] plugins share the same namespace (e.g `font_family` and `font_weight` share the same `font` namespace),
you need to fill [`Arbitrary::disambiguate`] with an [`ArbitraryDisambiguate`] structure.

This structure contains a CSS type which is compared to the CSS type inferred when parsing. If both types
match, the arbitrary plugin will be used (e.g `font-[Roboto]` will have an inferred type of [`CssType::FontFamilyName`]
which corresponds to `font_family` because it sets `matched: &[CssType::FontFamilyName]`).

When an arbitrary value does not have a clear type (e.g `font-[var(--font-weight)]` or `font-[initial]`),
the user needs to specify an arbitrary hint suffixed by `:`. This hint string will be parsed and compared
with the CSS type of this structure, just like the inferred type.

If [`Arbitrary::disambiguate`] is left with the default value, it will always match the value.
This behavior can be used to define a plugin which is used by-default when no other plugins match
the type of the CSS value, **but it should be registered last** otherwise all values will be handled
by it.

### Example

```
use encre_css::{Config, generate};
use encre_css::prelude::build_plugin::*;

pub(crate) const PLUGIN_ARBITRARY_COLOR: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "custom-stroke",
    prop: SingleProp("stroke"),
    disambiguate: Some(ArbitraryDisambiguate {
        matched: &[CssType::Color],
        separation: ArbitraryDisambiguateSeparation::None,
    }),
    ..Arbitrary::default()
});

const PLUGIN_ARBITRARY_WIDTH: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "custom-stroke",
    prop: SingleProp("stroke-width"),
    disambiguate: Some(ArbitraryDisambiguate {
        matched: &[CssType::Length, CssType::Percentage, CssType::LineWidth, CssType::Number],
        separation: ArbitraryDisambiguateSeparation::None,
    }),
    ..Arbitrary::default()
});

let mut config = Config::default();
config.register_plugin(&PLUGIN_ARBITRARY_COLOR);
config.register_plugin(&PLUGIN_ARBITRARY_WIDTH);

let generated = generate([
    "custom-stroke-[red]", "custom-stroke-[color:var(--stroke-color)]",
    "custom-stroke-[2]", "custom-stroke-[length:var(--stroke-width)]",
], &config);

assert!(generated.ends_with(r".custom-stroke-\[color\:var\(--stroke-color\)\] {
  stroke: var(--stroke-color);
}

.custom-stroke-\[red\] {
  stroke: red;
}

.custom-stroke-\[2\] {
  stroke-width: 2;
}

.custom-stroke-\[length\:var\(--stroke-width\)\] {
  stroke-width: var(--stroke-width);
}"));
```
