use std::env;

pub fn configure(mut config: LuaConfig) {
    if let Ok(prefix) = env::var("LUA_CONF_PREFIX") {
        config.set_prefix(&prefix);
    }

    let bitsint = (0 as libc::c_int).count_zeros();

    let bits32 = config.emit_if_defined("LUA_32BITS");
    let c89_numbers = config.emit_if_defined("LUA_C89_NUMBERS");
    config.emit_if_defined("LUA_USE_C89");
    config.emit_if_defined("LUA_COMPAT_5_2");
    config.emit_if_defined("LUA_COMPAT_5_1");
    config.emit_if_defined("LUA_COMPAT_FLOATSTRING");
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
}

#[cfg(feature = "embedded-lua")]
pub struct LuaConfig<'a> {
    build: &'a mut cc::Build,
    prefix: Option<String>,
}

#[cfg(not(feature = "embedded-lua"))]
pub struct LuaConfig<'a> {
    prefix: Option<String>,
    _marker: ::std::marker::PhantomData<&'a ()>,
}

impl LuaConfig<'_> {
    #[cfg(feature = "embedded-lua")]
    pub fn new(build: &mut cc::Build) -> LuaConfig {
        LuaConfig {
            prefix: None,
            build: build,
        }
    }

    #[cfg(not(feature = "embedded-lua"))]
    pub fn new() -> LuaConfig<'static> {
        LuaConfig {
            prefix: None,
            _marker: ::std::marker::PhantomData,
        }
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

                #[cfg(feature = "embedded-lua")]
                self.build.define(key, None);
            }
            Some(v) => {
                println!("Lua build define: {}={}", &key, v);
                println!("cargo:rustc-cfg={}=\"{}\"", &key, v);

                #[cfg(feature = "embedded-lua")]
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
