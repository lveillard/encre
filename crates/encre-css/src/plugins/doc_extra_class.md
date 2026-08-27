Add a suffix string to the class selector of the generated CSS rule.

### Example

```
use encre_css::{Config, generate};
use encre_css::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Spacing(Spacing {
    namespace: "custom-divide",
    prop: SingleProp("margin-inline"),
    extra_class: Some(" > :not(:last-child)"),
    ..Spacing::default()
});

let mut config = Config::default();
config.register_plugin(&PLUGIN);

let generated = generate(["custom-divide-2"], &config);

assert!(generated.ends_with(".custom-divide-2 > :not(:last-child) {
  margin-inline: 0.5rem;
}"));
```
