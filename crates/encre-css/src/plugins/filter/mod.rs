//! Filter utilities
pub mod backdrop_blur;
// pub mod backdrop_brightness;
// pub mod backdrop_contrast;
// pub mod backdrop_filter;
// pub mod backdrop_grayscale;
// pub mod backdrop_hue_rotate;
// pub mod backdrop_invert;
// pub mod backdrop_saturate;
// pub mod backdrop_sepia;
// pub mod blur;
// pub mod brightness;
// pub mod contrast;
// pub mod drop_shadow;
// pub mod filter_type;
// pub mod grayscale;
// pub mod hue_rotate;
// pub mod invert;
// pub mod saturate;
// pub mod sepia;

const CSS_FILTER: &str = "filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);";
const CSS_BACKDROP_FILTER: [&str; 2] = ["-webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);", "backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);"];

#[cfg(test)]
mod tests {
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn blur() {
        assert_eq!(
            generate(["blur-md"], &base_config()),
            ".blur-md {
  --en-blur: blur(12px);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
        assert_eq!(
            generate(["blur-none"], &base_config()),
            ".blur-none {
  --en-blur: blur(0);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
        assert_eq!(
            generate(["blur-[42px]"], &base_config()),
            r".blur-\[42px\] {
  --en-blur: blur(42px);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn brightness() {
        assert_eq!(
            generate(["brightness-20"], &base_config()),
            ".brightness-20 {
  --en-brightness: brightness(0.2);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn contrast() {
        assert_eq!(
            generate(["contrast-20"], &base_config()),
            ".contrast-20 {
  --en-contrast: contrast(0.2);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn drop_shadow() {
        assert_eq!(
            generate(["drop-shadow-md"], &base_config()),
            ".drop-shadow-md {
  --en-drop-shadow: drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06));
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
        assert_eq!(
            generate(["drop-shadow-none"], &base_config()),
            ".drop-shadow-none {
  --en-drop-shadow: drop-shadow(0 0 #0000);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
        assert_eq!(
            generate(["drop-shadow-[42px_12em_#f0f]"], &base_config()),
            r".drop-shadow-\[42px_12em_\#f0f\] {
  --en-drop-shadow: drop-shadow(42px 12em #f0f);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn filter_type() {
        assert_eq!(
            generate(["filter"], &base_config()),
            ".filter {
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );

        assert_eq!(
            generate(["filter-none"], &base_config()),
            ".filter-none {
  filter: none;
}"
        );
    }

    #[test]
    fn grayscale() {
        assert_eq!(
            generate(["grayscale-20"], &base_config()),
            ".grayscale-20 {
  --en-grayscale: grayscale(0.2);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );

        assert_eq!(
            generate(["grayscale"], &base_config()),
            ".grayscale {
  --en-grayscale: grayscale(100%);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn hue_rotate() {
        assert_eq!(
            generate(["hue-rotate-170"], &base_config()),
            ".hue-rotate-170 {
  --en-hue-rotate: hue-rotate(170deg);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
        assert_eq!(
            generate(["-hue-rotate-170"], &base_config()),
            ".-hue-rotate-170 {
  --en-hue-rotate: hue-rotate(-170deg);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn invert() {
        assert_eq!(
            generate(["invert-20"], &base_config()),
            ".invert-20 {
  --en-invert: invert(0.2);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );

        assert_eq!(
            generate(["invert"], &base_config()),
            ".invert {
  --en-invert: invert(100%);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn saturate() {
        assert_eq!(
            generate(["saturate-20"], &base_config()),
            ".saturate-20 {
  --en-saturate: saturate(0.2);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn sepia() {
        assert_eq!(
            generate(["sepia-20"], &base_config()),
            ".sepia-20 {
  --en-sepia: sepia(0.2);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );

        assert_eq!(
            generate(["sepia"], &base_config()),
            ".sepia {
  --en-sepia: sepia(100%);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn backdrop_blur() {
        assert_eq!(
            generate(["backdrop-blur-md"], &base_config()),

            ".backdrop-blur-md {
  --en-backdrop-blur: blur(12px);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
        assert_eq!(
            generate(["backdrop-blur-none"], &base_config()),
            ".backdrop-blur-none {
  --en-backdrop-blur: blur(0);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
        assert_eq!(
            generate(["backdrop-blur-[42px]"], &base_config()),
            r".backdrop-blur-\[42px\] {
  --en-backdrop-blur: blur(42px);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }

    #[test]
    fn backdrop_brightness() {
        assert_eq!(
            generate(["backdrop-brightness-20"], &base_config()),
            ".backdrop-brightness-20 {
  --en-backdrop-brightness: brightness(0.2);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }

    #[test]
    fn backdrop_contrast() {
        assert_eq!(
            generate(["backdrop-contrast-20"], &base_config()),
            ".backdrop-contrast-20 {
  --en-backdrop-contrast: contrast(0.2);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }

    #[test]
    fn backdrop_filter() {
        assert_eq!(
            generate(["backdrop-filter"], &base_config()),
            ".backdrop-filter {
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );

        assert_eq!(
            generate(["backdrop-filter-none"], &base_config()),
            ".backdrop-filter-none {
  -webkit-backdrop-filter: none;
  backdrop-filter: none;
}"
        );
    }

    #[test]
    fn backdrop_grayscale() {
        assert_eq!(
            generate(["backdrop-grayscale-20"], &base_config()),
            ".backdrop-grayscale-20 {
  --en-backdrop-grayscale: grayscale(0.2);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );

        assert_eq!(
            generate(["backdrop-grayscale"], &base_config()),
            ".backdrop-grayscale {
  --en-backdrop-grayscale: grayscale(100%);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }

    #[test]
    fn backdrop_hue_rotate() {
        assert_eq!(
            generate(["backdrop-hue-rotate-170"], &base_config()),
            ".backdrop-hue-rotate-170 {
  --en-backdrop-hue-rotate: hue-rotate(170deg);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
        assert_eq!(
            generate(["-backdrop-hue-rotate-170"], &base_config()),
            ".-backdrop-hue-rotate-170 {
  --en-backdrop-hue-rotate: hue-rotate(-170deg);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }

    #[test]
    fn backdrop_invert() {
        assert_eq!(
            generate(["backdrop-invert-20"], &base_config()),
            ".backdrop-invert-20 {
  --en-backdrop-invert: invert(0.2);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );

        assert_eq!(
            generate(["backdrop-invert"], &base_config()),
            ".backdrop-invert {
  --en-backdrop-invert: invert(100%);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }

    #[test]
    fn backdrop_saturate() {
        assert_eq!(
            generate(["backdrop-saturate-20"], &base_config()),
            ".backdrop-saturate-20 {
  --en-backdrop-saturate: saturate(0.2);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }

    #[test]
    fn backdrop_sepia() {
        assert_eq!(
            generate(["backdrop-sepia-20"], &base_config()),
            ".backdrop-sepia-20 {
  --en-backdrop-sepia: sepia(0.2);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );

        assert_eq!(
            generate(["backdrop-sepia"], &base_config()),
            ".backdrop-sepia {
  --en-backdrop-sepia: sepia(100%);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }
}
