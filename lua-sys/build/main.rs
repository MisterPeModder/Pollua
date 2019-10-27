#[cfg(feature = "embedded-lua")]
extern crate cc;
extern crate libc;
#[cfg(all(not(target_env = "msvc"), feature = "system-lua"))]
extern crate pkg_config;
extern crate rustc_version;
#[cfg(all(target_env = "msvc", feature = "system-lua"))]
extern crate vcpkg;

mod luaconf;

fn main() {
    println!("cargo:rustc-cfg=lua_64_bits");
    check_features();
    check_rustc_version();
    println!("cargo:rerun-if-changed=src");

    #[cfg(all(feature = "embedded-lua", not(feature = "system-lua")))]
    use_embedded_lua();
    #[cfg(all(feature = "system-lua", not(feature = "embedded-lua")))]
    use_system_lua();
}

fn check_features() {
    if cfg!(not(any(feature = "embedded-lua", feature = "system-lua"))) {
        panic!("missing feature 'embedded-lua' or 'system-lua'");
    } else if cfg!(all(feature = "embedded-lua", feature = "system-lua")) {
        panic!("conflicting features: 'embedded-lua' and 'system-lua'");
    }
}

fn check_rustc_version() {
    if let rustc_version::Channel::Nightly = rustc_version::version_meta().unwrap().channel {
        println!("cargo:rustc-cfg=rust_nightly");
    }
}

#[cfg(all(feature = "embedded-lua", not(feature = "system-lua")))]
macro_rules! add_lua_sources {
    ($cfg:ident, $root:expr, [$($file:expr),*]) => {
        $($cfg.file(::std::path::Path::new($root).join($file)));*
    };
}

#[cfg(all(feature = "embedded-lua", not(feature = "system-lua")))]
fn use_embedded_lua() {
    use std::env;

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or("".to_string());
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or("".to_string());

    let mut cc_config = cc::Build::new();

    luaconf::configure(luaconf::LuaConfig::new(&mut cc_config));
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

    cc_config.include("embedded");
    add_lua_sources!(
        cc_config,
        "embedded",
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

#[cfg(all(feature = "system-lua", not(feature = "embedded-lua")))]
fn use_system_lua() {
    luaconf::configure(luaconf::LuaConfig::new());
    #[cfg(target_env = "msvc")]
    find_vcpkg();
    #[cfg(not(target_env = "msvc"))]
    find_pkg_config();
}

/// Attempts to find the Lua package with vcpkg.
///
/// panics if the package was not found.
#[cfg(all(
    target_env = "msvc",
    feature = "system-lua",
    not(feature = "embedded-lua")
))]
fn find_vcpkg() {
    vcpkg::Config::new()
        .emit_includes(true)
        .probe("lua")
        .expect("vcpkg did not find the lua package");
}

/// Attempts to find the Lua package using pkg-config.
///
/// panics if the package was not found.
#[cfg(all(
    not(target_env = "msvc"),
    feature = "system-lua",
    not(feature = "embedded-lua")
))]
fn find_pkg_config() {
    pkg_config::Config::new()
        .atleast_version("5.3")
        .probe("lua")
        .expect("pkg-config did not find the lua package");
}
