#![doc = include_str!("../README.md")]
//!
//! It's convenient to have a single module in your codebase that exports the
//! macros along with the generated structs.
//!
//! Let's assume that module will live at `src/css/mod.rs` and that the
//! code generated by `tailwindcss-to-rust` lives at `src/css/generated.rs`. The
//! contents of `mod.rs` will look like this:
//!
//! ```rust,ignore
#![doc = include_str!("../examples/css/mod.rs")]
//! ```
//!
//! See [the `examples` directory in the git
//! repo](https://github.com/houseabsolute/tailwindcss-to-rust/tree/master/macros/examples)
//! for all of the above example code.

pub mod to_option_vec_string;
pub use to_option_vec_string::ToOptionVecString;

/// Takes one or more class names and returns a single space-separated string.
///
/// This macro provides a bit of sugar.
///
/// It frees you from having to write something like this:
///
/// ```rust,ignore
/// let classes = [C.lay.flex, C.fg.flex_col].join(" ");
/// ```
///
/// It also offers a lot of flexibility in what types it accepts, so you can
/// use any of the following as arguments to `C!`:
///
/// * `&str`
/// * `String`
/// * `&String`
/// * `Option<T>` and `&Option<T>` where `T` is any of the above.
/// * `Vec<T>`, `&Vec<T>`, and `&[T]` where `T` is any of the above.
#[macro_export]
macro_rules! C {
    ( $($class:expr $(,)?)+ ) => {
        {
            // [
            // $(
            //     $class.to_option_vec_string(),
            // )*
            // ].into_iter().filter_map(Option::is_some).flatten().join(" ")
            let mut all_classes = vec![];
            $(
                $crate::_push_all_strings(&mut all_classes, $class.to_option_vec_string());
            )*
            all_classes.join(" ")
        }
    };
}

/// Variant of the [`C!`] macro for use with Dioxus `class` attributes.
///
/// This works exactly like [`C!`] but it is designed to work with Dioxus's
/// attributes.
///
/// If you want to import this as `C!` just write this:
///
/// ```rust,ignore
/// use tailwindcss_to_rust_macros::DC as C;
///
/// div {
///    class: DC![C.typ.text_lg],
///    ...
/// }
/// ```
#[macro_export]
macro_rules! DC {
    ( $($class:expr $(,)?)+ ) => {
        {
            let mut all_classes = vec![];
            $(
                $crate::_push_all_strings(&mut all_classes, $class.to_option_vec_string());
            )*
            format_args!("{}", all_classes.join(" "))
        }
    };
}

/// Takes one or more tailwind modifier names and a class name, returning a single colon-separated string.
///
/// This works exactly like [`C!`] but it expects one or more modifier names
/// like "lg" or "hover", followed by a single class name.
///
/// ```rust,ignore
/// let classes = [
///     C.flex_and_grid.grid_cols_3,
///     M![M.lg, C.fg.grid_cols_6],
/// ].join(" ");
/// // classes is "grid-cols-3 lg:grid-cols-6"
/// ```
#[macro_export]
macro_rules! M {
    ( $($modifier:expr $(,)?)* ) => {
        {
            let mut all_modifiers = vec![];
            $(
                $crate::_push_all_strings(&mut all_modifiers, $modifier.to_option_vec_string());
            )*
            all_modifiers.join(":")
        }
    };
}

/// This is for use by the macros. Please don't use it yourself.
pub fn _push_all_strings(all_strings: &mut Vec<String>, classes: Option<Vec<String>>) {
    if let Some(classes) = classes {
        all_strings.append(
            &mut classes
                .into_iter()
                .filter(|c| !c.is_empty())
                .collect::<Vec<_>>(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(C![""], "");
        assert_eq!(C!["x"], "x");

        let x = "x";
        let y = "y";
        assert_eq!(C![x, y], "x y");
        assert_eq!(C![x, y, "z"], "x y z");

        assert_eq!(C![M!["md", "x"]], "md:x");
        assert_eq!(C![M!["md", "hover", "x"]], "md:hover:x");
        assert_eq!(C![M!["md", "x"], M!["hover", "y"]], "md:x hover:y");
        assert_eq!(C![x, M!["md", y], M!["hover", "z"]], "x md:y hover:z");

        let z = "z".to_string();
        assert_eq!(C![x, y, z, "foo"], "x y z foo");

        let z = "z".to_string();
        assert_eq!(C![M![x, y, z, "foo"]], "x:y:z:foo");
    }

    // These tests were copied from Seed (https://github.com/seed-rs/seed) and
    // then modified.
    //
    // Copyright 2019 DavidOConnor <david.alan.oconnor@gmail.com>
    //
    // Licensed under the MIT license only.

    // --- Texts ---

    #[test]
    fn to_option_vec_string_ref_str() {
        let text: &str = "foo";
        assert_eq!(C![text], "foo");
        assert_eq!(M![text], "foo");
    }

    #[test]
    fn to_option_vec_string_ref_str_empty() {
        let text: &str = "";
        assert!(C![text].is_empty());
        assert!(M![text].is_empty());
    }

    #[test]
    fn to_option_vec_string_string() {
        let text = String::from("bar");
        assert_eq!(C![text], "bar");
        let text = String::from("bar");
        assert_eq!(M![text], "bar");
    }

    #[test]
    fn to_option_vec_string_ref_string() {
        let text = &String::from("ref_bar");
        assert_eq!(C![text], "ref_bar");
        let text = &String::from("ref_bar");
        assert_eq!(M![text], "ref_bar");
    }

    // --- Containers ---

    #[test]
    fn to_option_vec_string_vec() {
        let vec: Vec<&str> = vec!["foo_1", "foo_2"];
        assert_eq!(C![vec], "foo_1 foo_2");
        let vec: Vec<&str> = vec!["foo_1", "foo_2"];
        assert_eq!(M![vec], "foo_1:foo_2");
    }

    #[test]
    fn to_option_vec_string_ref_vec() {
        let vec: &Vec<&str> = &vec!["foo_1", "foo_2"];
        assert_eq!(C![vec], "foo_1 foo_2");
        let vec: &Vec<&str> = &vec!["foo_1", "foo_2"];
        assert_eq!(M![vec], "foo_1:foo_2");
    }

    #[test]
    fn to_option_vec_string_slice() {
        let slice: &[&str] = &["foo_1", "foo_2"];
        assert_eq!(C![slice], "foo_1 foo_2");
        assert_eq!(M![slice], "foo_1:foo_2");
    }

    #[test]
    fn to_option_vec_string_option_some() {
        let option: Option<&str> = Some("foo_opt");
        assert_eq!(C![option], "foo_opt");
        assert_eq!(M![option], "foo_opt");
    }

    #[test]
    fn to_option_vec_string_ref_option_some() {
        let option: &Option<&str> = &Some("foo_opt");
        assert_eq!(C![option], "foo_opt");
        assert_eq!(M![option], "foo_opt");
    }

    #[test]
    fn to_option_vec_string_option_none() {
        let option: Option<&str> = None;
        assert!(C![option].is_empty());
        assert!(M![option].is_empty());
    }

    #[test]
    fn to_option_vec_string_option_vec() {
        let option_vec: Option<Vec<&str>> = Some(vec!["foo_1", "foo_2"]);
        assert_eq!(C![option_vec], "foo_1 foo_2");
        let option_vec: Option<Vec<&str>> = Some(vec!["foo_1", "foo_2"]);
        assert_eq!(M![option_vec], "foo_1:foo_2");
    }

    // I wrote this to help debug an issue with a Dioxus application where
    // similar code was leading to memory errors in the generated WASM.
    #[test]
    fn with_fmt() {
        struct Classes {
            classes: String,
        }

        impl std::fmt::Display for Classes {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {}", self.classes, C!["foo", "bar" M!["md", "baz"]],)
            }
        }

        let classes = Classes {
            classes: "x y z".to_string(),
        };
        assert_eq!(format!("{classes}"), "x y z foo bar md:baz");
    }
}
