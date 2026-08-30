//! Effect utilities
pub mod background_blend_mode;
pub mod box_shadow;
pub mod box_shadow_color;
pub mod mix_blend_mode;
pub mod opacity;
pub mod text_shadow;
pub mod text_shadow_color;

#[cfg(test)]
mod tests {
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn background_blend_mode() {
        assert_eq!(
            generate(["bg-blend-multiply"], &base_config()),
            ".bg-blend-multiply {
  background-blend-mode: multiply;
}"
        );
        assert_eq!(
            generate(["bg-blend-luminosity"], &base_config()),
            ".bg-blend-luminosity {
  background-blend-mode: luminosity;
}"
        );
    }

    #[test]
    fn box_shadow() {
        assert_eq!(
            generate(["shadow-sm"], &base_config()),
            ".shadow-sm {
  --en-shadow: 0 1px 3px 0 var(--en-shadow-color, rgb(0 0 0 / 0.1)), 0 1px 2px -1px var(--en-shadow-color, rgb(0 0 0 / 0.1));
  box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);
}"
        );
        assert_eq!(
            generate(["shadow-2xl"], &base_config()),
            ".shadow-2xl {
  --en-shadow: 0 25px 50px -12px var(--en-shadow-color, rgb(0 0 0 / 0.25));
  box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);
}"
        );
        assert_eq!(
            generate(["shadow-none"], &base_config()),
            ".shadow-none {
  --en-shadow: 0 0 #0000;
  box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);
}"
        );
        assert_eq!(
            generate(["shadow-[12px_3rem_12rem_2em_#f00]"], &base_config()),
            r".shadow-\[12px_3rem_12rem_2em_\#f00\] {
  --en-shadow: 12px 3rem 12rem 2em var(--en-shadow-color, #f00);
  box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);
}"
        );

        assert_eq!(
            generate(["inset-shadow-sm"], &base_config()),
            r".inset-shadow-sm {
  --en-inset-shadow: 0 2px 4px var(--en-inset-shadow-color, rgb(0 0 0 / 0.05));
  box-shadow: var(--en-inset-shadow), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow, 0 0 #0000);
}"
        );

        assert_eq!(
            generate(
                ["inset-shadow-[0_2px_2px_rgb(0_0_0_/_0.1)]"],
                &base_config()
            ),
            r".inset-shadow-\[0_2px_2px_rgb\(0_0_0_\/_0\.1\)\] {
  --en-inset-shadow: 0 2px 2px var(--en-inset-shadow-color, rgb(0 0 0 / 0.1));
  box-shadow: var(--en-inset-shadow), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow, 0 0 #0000);
}"
        );
    }

    #[test]
    fn box_shadow_color() {
        assert_eq!(
            generate(["shadow-red-400"], &base_config()),
            ".shadow-red-400 {
  --en-shadow-color: oklch(70.4% .191 22.216);
}"
        );
        assert_eq!(
            generate(["shadow-[rgb(12,12,12)]"], &base_config()),
            r".shadow-\[rgb\(12\,12\,12\)\] {
  --en-shadow-color: rgb(12,12,12);
}"
        );
        assert_eq!(
            generate(["inset-shadow-red-400"], &base_config()),
            ".inset-shadow-red-400 {
  --en-inset-shadow-color: oklch(70.4% .191 22.216);
}"
        );
    }

    #[test]
    fn text_shadow() {
        assert_eq!(
            generate(["text-shadow-2xs"], &base_config()),
            ".text-shadow-2xs {
  text-shadow: 0px 1px 0px var(--en-text-shadow-color, rgb(0 0 0 / 0.15));
}"
        );
        assert_eq!(
            generate(["text-shadow-sm"], &base_config()),
            ".text-shadow-sm {
  text-shadow: 0px 1px 0px var(--en-text-shadow-color, rgb(0 0 0 / 0.075)), 0px 1px 1px var(--en-text-shadow-color, rgb(0 0 0 / 0.075)), 0px 2px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.075));
}"
        );
        assert_eq!(
            generate(["text-shadow-none"], &base_config()),
            ".text-shadow-none {
  text-shadow: none;
}"
        );
        assert_eq!(
            generate(["text-shadow-[12px_3rem_12rem_#f00]"], &base_config()),
            r".text-shadow-\[12px_3rem_12rem_\#f00\] {
  text-shadow: 12px 3rem 12rem var(--en-text-shadow-color, #f00);
}"
        );
    }

    #[test]
    fn text_shadow_color() {
        assert_eq!(
            generate(["text-shadow-red-400"], &base_config()),
            ".text-shadow-red-400 {
  --en-text-shadow-color: oklch(70.4% .191 22.216);
}"
        );
        assert_eq!(
            generate(["text-shadow-[rgb(12,12,12)]"], &base_config()),
            r".text-shadow-\[rgb\(12\,12\,12\)\] {
  --en-text-shadow-color: rgb(12,12,12);
}"
        );
    }

    #[test]
    fn mix_blend_mode() {
        assert_eq!(
            generate(["mix-blend-multiply"], &base_config()),
            ".mix-blend-multiply {
  mix-blend-mode: multiply;
}"
        );
        assert_eq!(
            generate(["mix-blend-plus-lighter"], &base_config()),
            ".mix-blend-plus-lighter {
  mix-blend-mode: plus-lighter;
}"
        );
    }

    #[test]
    fn opacity() {
        assert_eq!(
            generate(["opacity-12"], &base_config()),
            ".opacity-12 {
  opacity: 0.12;
}"
        );
    }
}
