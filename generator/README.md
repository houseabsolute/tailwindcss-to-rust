The `tailwindcss-to-rust` tool reads a Tailwind CSS file and generates Rust
code from it.

The generated code allows you to use Tailwind CSS classes in your Rust
frontend code with compile-time checking of names and code completion for
class names. It also generates code for the full list of Tailwind modifiers
like `lg`, `hover`, etc.

Here's a quick start recipe:

1. [Install the `tailwindcss` CLI
   tool](https://tailwindcss.com/docs/installation). You can install it with
   `npm` or `npx`, or you can [download a standalone binary from the
   tailwindcss repo](https://github.com/tailwindlabs/tailwindcss/releases).
2. Create a `tailwind.config.js` file with the tool by running:

   ```
   tailwindcss init --full
   ```

3. Generate your CSS by running in
   the directory that contains your `tailwind.config.js` file, by running:

   ```
   tailwindcss --output path/to/tailwind.css
   ```

4. Install this tool by running:

   ```
   cargo install tailwindcss-to-rust
   ```

5. Generate your Rust code by running:

   ```
   tailwindcss-to-rust --css path/to/tailwind.css --output src/css/generated.rs
   ```

6. Check out the [tailwindcss-to-rust-macros
   crate](https://crates.io/crates/tailwindcss-to-rust-macros) for the most
   ergonomic way to use this generated code.

The generated names consist of all the class names present in the CSS file,
except names that start with a dash (`-`), names that contain pseudo-elements,
like `.placeholder-opacity-100::-moz-placeholder`, and names that contain
modifiers like `lg` or `hover`. Names are transformed into Rust identifiers
using the following algorithm:

* All backslash escapes are removed entirely, for example in ".inset-0\.5".
* All dashes (`-`) become an `underscore`.
* All period (`.`) become `_p_`, so `.inset-2\.5` becomes `inset_2_p_5`.
* All forward slashes (`/`) become `_of_`, so `.inset-2\/4` becomes
  `inset_2_of_4`.
* If a name *starts* with a `2`, as in `2xl`, it becomes `two_`, so the `2xl`
  modifier becomes `two_xl`.

The generated code consists of a set of structs and const variables containing
those structs. There is one struct for the modifiers and then many structs for
each group of classes. The class groups come from the Tailwind documentation's
left nav heading, so we have groups like "Layout", "Flexbox & Grid", etc. Any
custom CSS classes will end up in the "Unknown" group (Todo: a mechanism for
specifying what group custom classes go in).

Here's an example of the `Aspect` struct:

```rust
#[derive(Clone, Copy)]
pub(crate) struct Aspect {
    pub(crate) aspect_h_auto: &'static str,
    pub(crate) aspect_h_square: &'static str,
    pub(crate) aspect_h_video: &'static str,
    pub(crate) aspect_none: &'static str,
    pub(crate) aspect_w_auto: &'static str,
    pub(crate) aspect_w_square: &'static str,
    pub(crate) aspect_w_video: &'static str,
}
```

The generated `const` for that struct looks like this:

```rust
pub(crate) const ASPECT: Aspect = Aspect {
    aspect_h_auto: "aspect-h-auto",
    aspect_h_square: "aspect-h-square",
    aspect_h_video: "aspect-h-video",
    aspect_none: "aspect-none",
    aspect_w_auto: "aspect-w-auto",
    aspect_w_square: "aspect-w-square",
    aspect_w_video: "aspect-w-video",
};
```

The best way to understand the generated structs and consts is to simply open
the generated code file in your editor and look at it.

Then you can import these consts in your code and use them to refer to
Tailwind CSS class names with compile time checking:

```rust
element.set_class(ASPECT.aspect_h_auto);
```
