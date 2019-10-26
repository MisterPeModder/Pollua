# Lua-sys

## Building

Cargo features:
- **std**: Uses stdlib.
- **va-list**: Defines bindings for functions that have `va-list` in their arguments,
  a dependency on the [va_list](https://crates.io/crates/va_list) crate.
- **embedded-lua**: Links againsts the embedded Lua library.
- **system-lua**: Attempts to link against the system Lua library.
  conflicts with `embedded-lua`

Features `std`, `embedded-lua` and `va-list` are enabled by default

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
