use crate::*;

use core::iter::{Product, Sum};
use core::ops::*;
use core::str::FromStr;

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

    /// Returns the corresponding code for this value type.
    pub(crate) fn code(self) -> libc::c_int {
        match self {
            ValueType::Nil => sys::LUA_TNIL,
            ValueType::Boolean => sys::LUA_TBOOLEAN,
            ValueType::Number => sys::LUA_TNUMBER,
            ValueType::String => sys::LUA_TSTRING,
            ValueType::Function => sys::LUA_TFUNCTION,
            ValueType::LightUserdata => sys::LUA_TLIGHTUSERDATA,
            ValueType::Userdata => sys::LUA_TUSERDATA,
            ValueType::Thread => sys::LUA_TTHREAD,
            ValueType::Table => sys::LUA_TTABLE,
        }
    }
}

/// A type that can be pushed onto the stack.
pub trait Value: Sized + private::Sealed {
    /// Returns the type of this value.
    fn value_type() -> ValueType;
    /// Pushes the value onto the stack.
    fn push(self, thread: &mut Thread);
    /// Gets the value at the top of the stack without checking the type of the top value.
    ///
    /// # Safety
    /// Behavior is undefined if the value type at the top of the stack
    /// does not match the type returned by [`value_type`].
    ///
    /// [`value_type`]: #method.value_type
    unsafe fn get_unchecked(thread: &mut Thread) -> Self;
    /// Gets the value at the top of the stack and pops it.
    #[inline]
    fn get(thread: &mut Thread) -> Option<Self> {
        unsafe {
            if sys::lua_type(thread.as_raw().as_ptr(), -1) == Self::value_type().code() {
                Some(Self::get_unchecked(thread))
            } else {
                None
            }
        }
    }
}

/// A Lua floating-point number.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LuaNumber {
    value: sys::lua_Number,
}

impl Value for LuaNumber {
    #[inline(always)]
    fn value_type() -> ValueType {
        ValueType::Number
    }

    fn push(self, thread: &mut Thread) {
        unsafe { sys::lua_pushnumber(thread.as_raw().as_ptr(), self.value) }
    }

    unsafe fn get_unchecked(thread: &mut Thread) -> LuaNumber {
        let n = LuaNumber {
            value: sys::lua_tonumber(thread.as_raw().as_ptr(), -1),
        };
        sys::lua_pop(thread.as_raw().as_ptr(), 1);
        n
    }
}

impl fmt::Display for LuaNumber {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl fmt::LowerExp for LuaNumber {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl fmt::UpperExp for LuaNumber {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl Default for LuaNumber {
    /// Returns the default value of `0.0`
    #[inline]
    fn default() -> LuaNumber {
        LuaNumber { value: 0.0 }
    }
}

impl From<f32> for LuaNumber {
    /// Converts `f32` to `LuaNumber` losslessly.
    #[inline]
    fn from(v: f32) -> LuaNumber {
        LuaNumber {
            value: v as sys::lua_Number,
        }
    }
}

impl From<f64> for LuaNumber {
    /// Converts `f64` to `LuaNumber`,
    /// truncation may happen depending on the size of [`lua_sys::lua_Number`].
    #[inline]
    fn from(v: f64) -> LuaNumber {
        LuaNumber {
            value: v as sys::lua_Number,
        }
    }
}

impl Into<f32> for LuaNumber {
    /// Converts `LuaNumber` to `f64`,
    /// truncation may happen depending on the size of [`lua_sys::lua_Number`].
    #[inline]
    fn into(self) -> f32 {
        self.value as f32
    }
}

impl Into<f64> for LuaNumber {
    /// Converts `LuaNumber` to `f64` losslessly.
    #[inline]
    fn into(self) -> f64 {
        self.value as f64
    }
}

impl FromStr for LuaNumber {
    type Err = core::num::ParseFloatError;
    #[inline]
    fn from_str(src: &str) -> Result<LuaNumber, Self::Err> {
        Ok(LuaNumber {
            value: sys::lua_Number::from_str(src)?,
        })
    }
}

// Implements binary operations for $type.
macro_rules! number_binop {
    ($type:ty, $trait:ident, $fname:ident, $trait_assign:ident, $fname_assign:ident) => {
        impl $trait<$type> for $type {
            type Output = $type;
            #[inline(always)]
            fn $fname(self, other: $type) -> $type {
                <$type>::from($trait::$fname(self.value, other.value))
            }
        }
        impl $trait<&$type> for $type {
            type Output = <$type as $trait<$type>>::Output;
            #[inline(always)]
            fn $fname(self, other: &$type) -> <$type as $trait<$type>>::Output {
                self.$fname(*other)
            }
        }
        impl $trait_assign<$type> for $type {
            #[inline(always)]
            fn $fname_assign(&mut self, rhs: $type) {
                self.value.$fname_assign(rhs.value);
            }
        }
        impl $trait_assign<&$type> for $type {
            #[inline(always)]
            fn $fname_assign(&mut self, rhs: &$type) {
                self.$fname_assign(*rhs);
            }
        }
    };
}

number_binop!(LuaNumber, Add, add, AddAssign, add_assign);
number_binop!(LuaNumber, Sub, sub, SubAssign, sub_assign);
number_binop!(LuaNumber, Mul, mul, MulAssign, mul_assign);
number_binop!(LuaNumber, Div, div, DivAssign, div_assign);
number_binop!(LuaNumber, Rem, rem, RemAssign, rem_assign);

// Implements unary operations for $type.
macro_rules! number_unop {
    ($type:ty, $trait:ident, $fname:ident) => {
        impl $trait for $type {
            type Output = $type;
            #[inline(always)]
            fn $fname(self) -> $type {
                <$type>::from(self.value.$fname())
            }
        }
        impl $trait for &$type {
            type Output = <$type as $trait>::Output;
            #[inline(always)]
            fn $fname(self) -> <$type as $trait>::Output {
                <$type>::from(self.value.$fname())
            }
        }
    };
}

number_unop!(LuaNumber, Neg, neg);

macro_rules! number_sum {
    ($type:ty, $trait:ident, $fname:ident, $base:ty) => {
        impl $trait<$type> for $type {
            #[inline(always)]
            fn $fname<I>(iter: I) -> $type
            where
                I: Iterator<Item = $type>,
            {
                <$type>::from(<$base as $trait<$base>>::$fname(iter.map(<$type>::into)))
            }
        }
        impl<'a> $trait<&'a $type> for $type {
            #[inline(always)]
            fn $fname<I>(iter: I) -> $type
            where
                I: Iterator<Item = &'a $type>,
            {
                <$type>::from(<$base as $trait<$base>>::$fname(
                    iter.copied().map(<$type>::into),
                ))
            }
        }
    };
}

number_sum!(LuaNumber, Product, product, sys::lua_Number);
number_sum!(LuaNumber, Sum, sum, sys::lua_Number);

mod private {
    use super::*;
    pub trait Sealed {}

    impl Sealed for LuaNumber {}
}
