## v0.1.4 - 2022-04-24

- Added a new `--tailwindcss` argument. This can be used to provide the path
  to the `tailwindcss` executable.

## v0.1.3 - 2022-02-17

- Documentation fixes.

## v0.1.2 - 2022-02-14

- Fixed the handling of CSS class names with periods and forward slashes. The
  generated Rust code included an escape for these names, so they'd end up as
  things like `w-0\.5` or `w-3\/5` in your HTML, which is wrong. They should
  not have a backslash escape in the generated HTML. Note that this also means
  that the tailwind extractor code in the generator's `README.md` was wrong as
  well. It has also been fixed.

## v0.1.1 - 2022-02-08

- Fixed the repository metadata for the crate. Thanks to @overlisted on the
  Dioxus Discord for pointing this out.

## v0.1.0 - 2022-02-08

- First release upon an unsuspecting world.
