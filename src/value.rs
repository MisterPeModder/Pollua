pub use crate::*;

/// Lua value type.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ValueType {
    Nil,
    Boolean,
    Number,
    String,
    Function,
    LightUserdata,
    Userdata,
    Thread,
    Table,
}

impl ValueType {
    /// Creates a `ValueType` from an int.
    #[inline]
    pub(crate) fn from_code(code: libc::c_int) -> Option<ValueType> {
        ValueType::from_code_ref(code).copied()
    }

    /// Creates a `ValueType` from an int.
    pub(crate) fn from_code_ref(code: libc::c_int) -> Option<&'static ValueType> {
        match code {
            sys::LUA_TNIL => Some(&ValueType::Nil),
            sys::LUA_TBOOLEAN => Some(&ValueType::Boolean),
            sys::LUA_TNUMBER => Some(&ValueType::Number),
            sys::LUA_TSTRING => Some(&ValueType::String),
            sys::LUA_TFUNCTION => Some(&ValueType::Function),
            sys::LUA_TLIGHTUSERDATA => Some(&ValueType::LightUserdata),
            sys::LUA_TUSERDATA => Some(&ValueType::Userdata),
            sys::LUA_TTHREAD => Some(&ValueType::Thread),
            sys::LUA_TTABLE => Some(&ValueType::Table),
            _ => None,
        }
    }
}
