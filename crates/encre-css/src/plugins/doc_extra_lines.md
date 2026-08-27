Add one or several extra CSS line(s) **inside** the CSS rule generated for the utility class.

### Example

```
use encre_css::{Config, generate};
use encre_css::prelude::build_plugin::*;

const PLUGIN: StaticPlugin = Plugin::Spacing(Spacing {
    namespace: "custom-translate-x",
    prop: SingleProp("--translate-x"),
    extra_lines: Some(&["transform: translate(var(--translate-x), 12px);"]),
    ..Spacing::default()
});

let mut config = Config::default();
config.register_plugin(&PLUGIN);

let generated = generate(["custom-translate-x-8"], &config);

assert!(generated.ends_with(".custom-translate-x-8 {
  --translate-x: 2rem;
  transform: translate(var(--translate-x), 12px);
}"));
```
