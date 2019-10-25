/*
 * Copyright (c) 2019 Yanis Guaye
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http: //www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[cfg(feature = "embedded-lua")]
extern crate cc;
#[cfg(all(not(target_env = "msvc"), feature = "system-lua"))]
extern crate pkg_config;
#[cfg(all(target_env = "msvc", feature = "system-lua"))]
extern crate vcpkg;

fn main() {
    #[cfg(feature = "embedded-lua")]
    use_embedded_lua();
    #[cfg(feature = "system-lua")]
    use_system_lua();
    copy_pregenerated_mappings();
}

#[cfg(feature = "embedded-lua")]
macro_rules! add_lua_sources {
    ($cfg:ident, $root:expr, [$($file:expr),*]) => {
        $($cfg.file(::std::path::Path::new($root).join($file)));*
    };
}

#[cfg(feature = "embedded-lua")]
fn use_embedded_lua() {
    use std::env;

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or("".to_string());
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or("".to_string());

    let mut cc_config = cc::Build::new();

    if let Some(define) = match (target_os.as_str(), target_family.as_str()) {
        ("linux", _) => Some("LUA_USE_LINUX"),
        ("macos", _) => Some("LUA_USE_MACOSX"),
        (_, "unix") => Some("LUA_USE_POSIX"),
        (_, "windows") => Some("LUA_USE_WINDOWS"),
        (_, _) => None,
    } {
        cc_config.define(define, None);
    };

    if cfg!(debug_assertions) {
        cc_config.define("LUA_USE_API_CHECK", None);
    }

    cc_config.include("src/embedded");
    add_lua_sources!(
        cc_config,
        "src/embedded",
        [
            "lapi.c",
            "lauxlib.c",
            "lbaselib.c",
            "lbitlib.c",
            "lcode.c",
            "lcorolib.c",
            "lctype.c",
            "ldblib.c",
            "ldebug.c",
            "ldo.c",
            "ldump.c",
            "lfunc.c",
            "lgc.c",
            "linit.c",
            "liolib.c",
            "llex.c",
            "lmathlib.c",
            "lmem.c",
            "loadlib.c",
            "lobject.c",
            "lopcodes.c",
            "loslib.c",
            "lparser.c",
            "lstate.c",
            "lstring.c",
            "lstrlib.c",
            "ltable.c",
            "ltablib.c",
            "ltm.c",
            "lundump.c",
            "lutf8lib.c",
            "lvm.c",
            "lzio.c"
        ]
    );

    cc_config.compile("liblua5.3.a");
}

#[cfg(feature = "system-lua")]
fn use_system_lua() {
    #[cfg(target_env = "msvc")]
    find_vcpkg();
    #[cfg(not(target_env = "msvc"))]
    find_pkg_config();
}

/// Attempts to find the Lua package with vcpkg.
///
/// panics if the package was not found.
#[cfg(all(target_env = "msvc", feature = "system-lua"))]
fn find_vcpkg() {
    vcpkg::Config::new()
        .emit_includes(true)
        .probe("lua")
        .expect("vcpkg did not find the lua package");
}

/// Attempts to find the Lua package using pkg-config.
///
/// panics if the package was not found.
#[cfg(all(not(target_env = "msvc"), feature = "system-lua"))]
fn find_pkg_config() {
    pkg_config::Config::new()
        .atleast_version("5.3")
        .probe("lua")
        .expect("pkg-config did not find the lua package");
}

/// Copies the lua bindings file in the crate root
/// to the output directory.
fn copy_pregenerated_mappings() {
    use std::env;
    use std::fs;
    use std::path::PathBuf;

    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::copy(
        crate_path.join("lua_bindings.rs"),
        out_path.join("lua_bindings.rs"),
    )
    .expect("Couldn't find pregenerated bindings!");
}
