//! Transform utilities
pub mod perspective;
pub mod perspective_origin;
pub mod rotate;
pub mod scale;
pub mod skew;
pub mod transform_origin;
pub mod transform_type;
pub mod translate;

const CSS_TRANSFORM: &str = "transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));";

#[cfg(test)]
mod tests {
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn perspective() {
        assert_eq!(
            generate(["perspective-distant"], &base_config()),
            ".perspective-distant {
  perspective: 1200px;
}"
        );
        assert_eq!(
            generate(["perspective-[0.1rem]"], &base_config()),
            r".perspective-\[0\.1rem\] {
  perspective: 0.1rem;
}"
        );
    }

    #[test]
    fn perspective_origin() {
        assert_eq!(
            generate(["perspective-origin-top-left"], &base_config()),
            ".perspective-origin-top-left {
  perspective-origin: top left;
}"
        );
        assert_eq!(
            generate(["perspective-origin-[0.1rem_center]"], &base_config()),
            r".perspective-origin-\[0\.1rem_center\] {
  perspective-origin: 0.1rem center;
}"
        );
    }

    #[test]
    fn rotate() {
        assert_eq!(generate(["rotate-20"], &base_config()), ".rotate-20 {
  --en-rotate-x: 20deg;
  --en-rotate-y: 20deg;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(
            generate(["rotate-[1.25turn]"], &base_config()),
            r".rotate-\[1\.25turn\] {
  --en-rotate-x: 1.25turn;
  --en-rotate-y: 1.25turn;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}"
        );

        assert_eq!(generate(["rotate-x-20"], &base_config()), ".rotate-x-20 {
  --en-rotate-x: 20deg;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(
            generate(["rotate-x-[1.25turn]"], &base_config()),
            r".rotate-x-\[1\.25turn\] {
  --en-rotate-x: 1.25turn;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}"
        );

        assert_eq!(generate(["rotate-y-20"], &base_config()), ".rotate-y-20 {
  --en-rotate-y: 20deg;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(
            generate(["rotate-y-[1.25turn]"], &base_config()),
            r".rotate-y-\[1\.25turn\] {
  --en-rotate-y: 1.25turn;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}"
        );

        assert_eq!(generate(["rotate-z-20"], &base_config()), ".rotate-z-20 {
  --en-rotate-z: 20deg;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(
            generate(["rotate-z-[1.25turn]"], &base_config()),
            r".rotate-z-\[1\.25turn\] {
  --en-rotate-z: 1.25turn;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}"
        );
    }

    #[test]
    fn scale() {
        assert_eq!(generate(["scale-150"], &base_config()), ".scale-150 {
  --en-scale-x: 1.5;
  --en-scale-y: 1.5;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(generate(["scale-x-20"], &base_config()), ".scale-x-20 {
  --en-scale-x: 0.2;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(generate(["scale-y-45"], &base_config()), ".scale-y-45 {
  --en-scale-y: 0.45;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(
            generate(["scale-y-[88.42%]"], &base_config()),
            r".scale-y-\[88\.42\%\] {
  --en-scale-y: 88.42%;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}"
        );
        assert_eq!(generate(["scale-z-45"], &base_config()), ".scale-z-45 {
  --en-scale-z: 0.45;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
    }

    #[test]
    fn skew() {
        assert_eq!(generate(["skew-x-20"], &base_config()), ".skew-x-20 {
  --en-skew-x: 20deg;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(generate(["skew-y-20"], &base_config()), ".skew-y-20 {
  --en-skew-y: 20deg;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(
            generate(["skew-x-[1.25turn]"], &base_config()),
            r".skew-x-\[1\.25turn\] {
  --en-skew-x: 1.25turn;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}"
        );
        assert_eq!(
            generate(["skew-y-[1.25turn]"], &base_config()),
            r".skew-y-\[1\.25turn\] {
  --en-skew-y: 1.25turn;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}"
        );
    }

    #[test]
    fn translate() {
        assert_eq!(generate(["translate-x-20"], &base_config()), ".translate-x-20 {
  --en-translate-x: 5rem;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(generate(["-translate-x-full"], &base_config()), ".-translate-x-full {
  --en-translate-x: -100%;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(generate(["-translate-x-20"], &base_config()), ".-translate-x-20 {
  --en-translate-x: -5rem;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(generate(["translate-x-auto"], &base_config()), ".translate-x-auto {
  --en-translate-x: auto;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(
            generate(["translate-x-[1.25%]"], &base_config()),
            r".translate-x-\[1\.25\%\] {
  --en-translate-x: 1.25%;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}"
        );

        assert_eq!(generate(["translate-y-20"], &base_config()), ".translate-y-20 {
  --en-translate-y: 5rem;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(generate(["-translate-y-full"], &base_config()), ".-translate-y-full {
  --en-translate-y: -100%;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(generate(["-translate-y-20"], &base_config()), ".-translate-y-20 {
  --en-translate-y: -5rem;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(generate(["translate-y-auto"], &base_config()), ".translate-y-auto {
  --en-translate-y: auto;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(
            generate(["translate-y-[1.25%]"], &base_config()),
            r".translate-y-\[1\.25\%\] {
  --en-translate-y: 1.25%;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}"
        );

        assert_eq!(generate(["translate-z-20"], &base_config()), ".translate-z-20 {
  --en-translate-z: 5rem;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(generate(["-translate-z-full"], &base_config()), ".-translate-z-full {
  --en-translate-z: -100%;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(generate(["-translate-z-20"], &base_config()), ".-translate-z-20 {
  --en-translate-z: -5rem;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(generate(["translate-z-auto"], &base_config()), ".translate-z-auto {
  --en-translate-z: auto;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(
            generate(["translate-z-[1.25%]"], &base_config()),
            r".translate-z-\[1\.25\%\] {
  --en-translate-z: 1.25%;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}"
        );
    }

    #[test]
    fn transform_origin() {
        assert_eq!(
            generate(["origin-center"], &base_config()),
            ".origin-center {
  transform-origin: center;
}"
        );
        assert_eq!(
            generate(["origin-top-left"], &base_config()),
            ".origin-top-left {
  transform-origin: top left;
}"
        );
        assert_eq!(
            generate(["origin-[bottom_right_60px]"], &base_config()),
            r".origin-\[bottom_right_60px\] {
  transform-origin: bottom right 60px;
}"
        );
        assert_eq!(
            generate(["origin-[-100%_40%]"], &base_config()),
            r".origin-\[-100\%_40\%\] {
  transform-origin: -100% 40%;
}"
        );
    }

    #[test]
    fn transform_type() {
        assert_eq!(generate(["transform-gpu"], &base_config()), ".transform-gpu {
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}");
        assert_eq!(
            generate(["transform-none"], &base_config()),
            ".transform-none {
  transform: none;
}"
        );
    }
}
