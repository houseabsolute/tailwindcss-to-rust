// This code was copied from Seed (https://github.com/seed-rs/seed) and
// modified.
//
// Copyright 2019 DavidOConnor <david.alan.oconnor@gmail.com>
//
// Licensed under the MIT license only.

//! Contains a trait to make the macros work with many types.
//!
//! ```rust,ignore
//! use tailwindcss_to_rust_macros::{C, M, ToOptionvecstring};
//! ```
/// You need to make sure this trait is imported by any code that wants to use
/// the `C!`, `DC!`, or `M!` macros.
pub trait ToOptionVecString {
    fn to_option_vec_string(self) -> Option<Vec<String>>;
}

// ------ Implementations ------

impl<T: ToOptionVecString + Clone> ToOptionVecString for &T {
    fn to_option_vec_string(self) -> Option<Vec<String>> {
        self.clone().to_option_vec_string()
    }
}

// --- Texts ---

impl ToOptionVecString for String {
    fn to_option_vec_string(self) -> Option<Vec<String>> {
        Some(vec![self])
    }
}

impl ToOptionVecString for &str {
    fn to_option_vec_string(self) -> Option<Vec<String>> {
        Some(vec![self.to_string()])
    }
}

// --- Containers ---

impl<T: ToOptionVecString> ToOptionVecString for Option<T> {
    fn to_option_vec_string(self) -> Option<Vec<String>> {
        self.and_then(ToOptionVecString::to_option_vec_string)
    }
}

impl<T: ToOptionVecString> ToOptionVecString for Vec<T> {
    fn to_option_vec_string(self) -> Option<Vec<String>> {
        let classes = self
            .into_iter()
            .filter_map(ToOptionVecString::to_option_vec_string)
            .flatten();
        Some(classes.collect())
    }
}

impl<T: ToOptionVecString + Clone> ToOptionVecString for &[T] {
    fn to_option_vec_string(self) -> Option<Vec<String>> {
        let classes = self
            .iter()
            .filter_map(ToOptionVecString::to_option_vec_string)
            .flatten();
        Some(classes.collect())
    }
}
