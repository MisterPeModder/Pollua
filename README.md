[![Actions Status]][Github-Actions]
Pollua
=========================

(WIP) rust bindings to Lua 5.3

## Building

### Using embedded Lua
On Windows and Linux:
```
cargo build
```
### Using system Lua

on Windows:
requires vcpkg

```
vcpkg install --triplet x64-windows lua
cargo build
```

### Cargo features:
- **std**: Uses stdlib.
- **system-lua**: Attempts to link against the system Lua library instead of the            embedded lua lib.
- **lua-compat**: Enables compatibilty for Lua versions 5.1 and 5.2.

Feature `std` is enabled by default.

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
