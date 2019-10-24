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

#[cfg(target_env = "msvc")]
extern crate vcpkg;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    find_lua();
    copy_pregenerated_mappings();
}

fn find_lua() {
    let host = env::var("HOST").expect("Cargo build scripts always have HOST");

    if find_vcpkg() {
        return;
    }

    if host.contains("windows-msvc") {
        panic!(format!(
            "
It looks like you are compiling for MSVC
but we could not find the Lua installation.
"
        ))
    } else if host.contains("windows-gnu") {
        panic!(format!(
            "
It looks like you are compiling for MinGW
but this platform is not supported (yet).

Please use MSVC instead.    
"
        ))
    } else {
        panic!(format!("{} is not supported (yet).", host))
    }
}

/// Attempts to find the Lua package with vcpkg.
///
/// returns `Ok(true)` if the package was found or `Err` otherwise.
#[cfg(target_env = "msvc")]
fn find_vcpkg() -> bool {
    match vcpkg::Config::new().emit_includes(true).probe("lua") {
        Ok(_) => true,
        Err(e) => panic!(format!(
            "error: vcpkg did not find the lua package: \n{}",
            e
        )),
    }
}

#[cfg(not(target_env = "msvc"))]
fn find_vcpkg() -> bool {
    false
}

fn copy_pregenerated_mappings() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    fs::copy(
        crate_path.join("lua_bindings.rs"),
        out_path.join("lua_bindings.rs"),
    )
    .expect("Couldn't find pregenerated bindings!");
}
