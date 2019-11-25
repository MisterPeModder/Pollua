# Lua-sys

[![Actions Status]][Github-Actions]
[![Crate]][Crates.io]
[![Documentation][Docs Badge]][Docs]

Lua 5.x bindings for the Rust programming language.

[Documentation][Docs].

## Release Support
The current supported releases of Lua are 5.1, 5.2 and 5.3. Lua 5.0 support upcoming.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
lua-sys = "^0.2.0"
```

## Crate Features

- **std**: Uses stdlib.
- **va-list**: Defines bindings for functions that have `va-list` in their arguments,
    a dependency on the [va_list](https://crates.io/crates/va_list) crate.
- **system-lua**: Attempts to link against the system Lua library instead of using the      embedded lua.
- **lua-compat**: Enables compatibilty for Lua versions 5.1 and 5.2.

Features `std` and `va-list` are enabled by default.

When feature `system-lua` is enabled, lua-sys will search for the Lua library using [pkg-config](https://github.com/rust-lang/pkg-config-rs) on Unix and [vcpkg](https://github.com/mcgoo/vcpkg-rs) on Windows.

## Lua Configuration

Properties of the Lua library can be changed by defining the following environment variables:
LUA_CONF_PREFIX
- `LUA_32BITS`
    Enables Lua with 32-bit integers and 32-bit floats.
- `LUA_C89_NUMBERS`
    Ensures that Lua uses the largest types available for C89.
- `LUA_USE_C89`
    Controls the use of non-ISO-C89 features.
- `LUA_NOCVTN2S`
    Define to turn off automatic coercion from numbers to strings.
- `LUA_NOCVTS2N`
    Define LUA_NOCVTS2N to turn off automatic coercion from strings to numbers.
- `LUA_INT_TYPE="LUA_INT_INT" | "LUA_INT_LONG" | "LUA_INT_LONGLONG"`
    Defines the type for Lua integers.
- `LUA_FLOAT_TYPE="LUA_FLOAT_FLOAT" | "LUA_FLOAT_DOUBLE" | "LUA_FLOAT_LONGDOUBLE"`
    Defines the type for Lua floats.
- `LUA_VERSION="<major>.<minor>.<patch>"`
    Sets the system Lua version, defaults to `"5.3.5"`.

Example:
```sh
$ export LUA_32BITS=1
$ export LUA_INT_TYPE="LUA_INT_INT"
$ export LUA_VERSION="5.2.0"
$ cargo build --no-default-features --features "system-lua va-list"
```

The `LUA_CONF_PREFIX` variable can be used to change the name of the above variable:
```sh
$ LUA_CONF_PREFIX="MY_CONF_" MY_CONF_LUA_USE_C89=1 cargo build
```

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

[Actions Status]: https://github.com/MisterPeModder/Pollua/workflows/CI/badge.svg
[Github-Actions]: https://github.com/MisterPeModder/Pollua/actions
[Crate]: https://img.shields.io/crates/v/lua-sys.svg
[Crates.io]: https://crates.io/crates/lua-sys
[Docs]: https://docs.rs/lua-sys
[Docs Badge]: https://docs.rs/lua-sys/badge.svg
