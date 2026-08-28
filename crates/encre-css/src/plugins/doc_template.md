Define a format string used to modify the generated CSS value…

In practice, you give a string containing a `{}` placeholder to this field (wrapped in the same
[`PropertyName`] variant as the `prop` field) and it will be replaced during CSS generation by the
value matched by the plugin kind and options.

For instance, it can be used to specify a CSS unit when using the [`Number`] plugin kind or to wrap
the value in a CSS function like `translate` or `blur`.

You should use the same property name variant as `prop`, otherwise the plugin matches will be
silently ignored. Check the examples below to see two correct usages.

### Example

```
use encre_css::{Config, generate};
use encre_css::prelude::build_plugin::*;

pub(crate) const PLUGIN_SINGLE_PROP: StaticPlugin = Plugin::Number(Number {
    namespace: "custom-stroke",
    prop: SingleProp("stroke-width"),
    template: Some(SingleProp("{}px")),
    ..Number::default()
});

pub(crate) const PLUGIN_MULTIPLE_PROPS: StaticPlugin = Plugin::Number(Number {
    namespace: "custom-move",
    prop: MultipleProps(&["translate", "rotate"]),
    template: Some(MultipleProps(&["{}px", "{}deg"])),
    ..Number::default()
});

let mut config = Config::default();
config.register_plugin(&PLUGIN_SINGLE_PROP);
config.register_plugin(&PLUGIN_MULTIPLE_PROPS);

let generated = generate(["custom-stroke-42", "custom-move-3"], &config);

assert!(generated.ends_with(".custom-stroke-42 {
  stroke-width: 42px;
}

.custom-move-3 {
  translate: 3px;
  rotate: 3deg;
}"));
```
