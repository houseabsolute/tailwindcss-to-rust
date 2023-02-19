## v0.3.0 - 2023-02-18

- The generated code now groups CSS classes into modules instead of
  structs. This prevents stack overflows that happened when the structs were
  put on the stack. Thanks to @mdochdev for identifying the issue and
  suggesting this fix. GH #4.

- Updated the class categories for the latest TailwindCSS version, 3.2.7.

## v0.2.0 - 2023-02-05

- Updated the class categories for the latest TailwindCSS version, 3.2.4.

- Updated the docs to make it clearer how to configure `tailwind.config.js`
  depending on whether or not you're using the macros, what templating system
  you're using, etc.

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
