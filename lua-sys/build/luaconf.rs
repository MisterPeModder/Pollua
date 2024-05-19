use std::env;

pub fn configure(mut config: LuaConfig) {
    if let Ok(prefix) = env::var("LUA_CONF_PREFIX") {
        config.set_prefix(&prefix);
    }

    let bitsint = (0 as libc::c_int).count_zeros();

    let bits32 = config.emit_if_defined("LUA_32BITS");
    let c89_numbers = config.emit_if_defined("LUA_C89_NUMBERS");
    config.emit_if_defined("LUA_USE_C89");
    config.emit_if_defined("LUA_NOCVTN2S");
    config.emit_if_defined("LUA_NOCVTS2N");

    let mut int_type;
    let mut float_type;

    if bits32 {
        if bitsint >= 32 {
            int_type = Some("LUA_INT_INT".to_string());
        } else {
            int_type = Some("LUA_INT_LONG".to_string());
        }
        float_type = Some("LUA_FLOAT_FLOAT".to_string());
    } else if c89_numbers {
        int_type = Some("LUA_INT_LONG".to_string());
        float_type = Some("LUA_FLOAT_DOUBLE".to_string());
    } else {
        int_type = Some("LUA_INT_LONGLONG".to_string());
        float_type = Some("LUA_FLOAT_DOUBLE".to_string());
    }

    if let Some(v) = config.env("LUA_INT_TYPE") {
        if v != "LUA_INT_INT" && v != "LUA_INT_LONG" && v != "LUA_INT_LONGLONG" {
            panic!(
                "LUA_FLOAT_TYPE must be of value \
                 LUA_INT_INT, LUA_INT_LONG or LUA_INT_LONGLONG"
            );
        }
        int_type = Some(v);
    }

    if let Some(v) = config.env("LUA_FLOAT_TYPE") {
        if v != "LUA_FLOAT_FLOAT" && v != "LUA_FLOAT_DOUBLE" && v != "LUA_FLOAT_LONGDOUBLE" {
            panic!(
                "LUA_FLOAT_TYPE must be of value \
                 LUA_FLOAT_FLOAT, LUA_FLOAT_DOUBLE or LUA_FLOAT_LONGDOUBLE"
            );
        }
        float_type = Some(v);
    }

    config.emit("LUA_INT_TYPE", int_type.as_ref().map(|x| &**x));
    config.emit("LUA_FLOAT_TYPE", float_type.as_ref().map(|x| &**x));

    emit_lua_version(&mut config);
}

const EMBEDDED_VERSION: (u32, u32, u32) = (5, 3, 5);

fn emit_lua_version(config: &mut LuaConfig) {
    use std::fs::File;
    use std::io::BufWriter;
    use std::io::Write;
    use std::path::Path;

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("lua_version.rs");
    let version_str = config.env("LUA_VERSION").or_else(|| config.version.clone());

    let (major, minor, patch) = match version_str {
        Some(v) => {
            let version = parse_lua_version(&v);
            #[cfg(not(feature = "system-lua"))]
            {
                println!(
                    "cargo:warning=Tried to override LUA_VERSION of embedded Lua from {}.{}.{} to {}.{}.{}",
                    EMBEDDED_VERSION.0,
                    EMBEDDED_VERSION.1,
                    EMBEDDED_VERSION.2,
                    version.0,
                    version.1,
                    version.2,
                );
                EMBEDDED_VERSION
            }
            #[cfg(feature = "system-lua")]
            {
                println!("Lua version: {}.{}.{}", version.0, version.1, version.2);
                version
            }
        }
        None => EMBEDDED_VERSION,
    };

    if major != 5 {
        panic!("LUA_VERSION major must be equal to 5");
    }

    // emits LUA_VERSION with values from major.0 to major.minor
    for m in 0..=minor {
        println!("cargo:rustc-cfg=LUA_VERSION=\"{}.{}\"", major, m);
    }

    let mut out =
        BufWriter::new(File::create(&path).expect(&format!("Could not create {}", path.display())));

    writeln!(out, "pub const VERSION_MAJOR: &str = \"{}\";", major).unwrap();
    writeln!(out, "pub const VERSION_MINOR: &str = \"{}\";", minor).unwrap();
    writeln!(out, "pub const VERSION_RELEASE: &str = \"{}\";", patch).unwrap();
    writeln!(
        out,
        "pub const VERSION_NUM: crate::lua_Number = {}.0;",
        major * 100 + minor
    )
    .unwrap();
    let version = format!("Lua {}.{}", major, minor);
    writeln!(out, "pub const VERSION: &str = \"{}\";", version).unwrap();
    let release = format!("{}.{}", &version, patch);
    writeln!(out, "pub const RELEASE: &str = \"{}\";", release).unwrap();
    writeln!(
        out,
        "pub const LUA_VERSUFFIX: &str = \"_{}_{}\";",
        major, minor
    )
    .unwrap();
}

fn parse_lua_version(version: &str) -> (u32, u32, u32) {
    let mut version = version.trim();
    if version.starts_with("Lua") {
        version = (&version[3..]).trim();
    }
    let mut split = version.split(".");
    let release = (
        split
            .next()
            .expect("missing major in LUA_VERSION")
            .parse()
            .expect("invalid major in LUA_VERSION"),
        split
            .next()
            .expect("missing minor in LUA_VERSION")
            .parse()
            .expect("invalid minor in LUA_VERSION"),
        split
            .next()
            .expect("missing patch in LUA_VERSION")
            .parse()
            .expect("invalid patch in LUA_VERSION"),
    );
    if split.next().is_some() {
        panic!("LUA_VERSION must be of format major.minor.patch");
    }
    release
}

#[cfg(not(feature = "system-lua"))]
pub struct LuaConfig<'a> {
    build: &'a mut cc::Build,
    prefix: Option<String>,
    version: Option<String>,
}

#[cfg(feature = "system-lua")]
pub struct LuaConfig<'a> {
    prefix: Option<String>,
    version: Option<String>,
    _marker: ::std::marker::PhantomData<&'a ()>,
}

impl LuaConfig<'_> {
    #[cfg(not(feature = "system-lua"))]
    pub fn new(build: &mut cc::Build) -> LuaConfig {
        LuaConfig {
            prefix: None,
            build: build,
            version: None,
        }
    }
    #[cfg(feature = "system-lua")]
    pub fn new() -> LuaConfig<'static> {
        LuaConfig {
            prefix: None,
            version: None,
            _marker: ::std::marker::PhantomData,
        }
    }

    pub fn set_version(&mut self, version: &str) {
        self.version = Some(version.to_owned());
    }

    pub fn set_prefix(&mut self, prefix: &str) {
        match &mut self.prefix {
            Some(p) => {
                p.clear();
                p.push_str(prefix);
            }
            None => self.prefix = Some(prefix.to_owned()),
        }
    }

    /// Gets the key and the prefix of an envionment variable.
    #[inline]
    pub fn env(&self, key: &str) -> Option<String> {
        match &self.prefix {
            Some(p) => env::var(format!("{}{}", p, key)),
            None => env::var(key),
        }
        .ok()
    }

    /// Emits a key-value pair as rust config and as a C define.
    pub fn emit(&mut self, key: &str, value: Option<&str>) {
        match value {
            None => {
                println!("lua build define: {}", &key);
                println!("cargo:rustc-cfg={}", &key);

                #[cfg(not(feature = "system-lua"))]
                self.build.define(key, None);
            }
            Some(v) => {
                println!("Lua build define: {}={}", &key, v);
                println!("cargo:rustc-cfg={}=\"{}\"", &key, v);

                #[cfg(not(feature = "system-lua"))]
                self.build.define(key, value);
            }
        }
    }

    /// Emits the key as a rust config key and C define
    /// if it is present in the environement.
    /// Does not emit the irinal value of the env variable.
    #[inline]
    pub fn emit_if_defined(&mut self, key: &str) -> bool {
        if self.env(key).is_some() {
            self.emit(key, None);
            true
        } else {
            false
        }
    }
}
