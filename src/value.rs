use crate::thread::{Thread, ThreadRef};

use std::{
    ascii,
    borrow::Cow,
    cmp::Ordering,
    fmt::{self, Pointer, Write},
    iter::{Product, Sum},
    num::ParseFloatError,
    ops::*,
    panic::{RefUnwindSafe, UnwindSafe},
    ptr::{self, NonNull},
    slice,
    str::{self, FromStr, Utf8Error},
};

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

/// Pushes a value onto the stack
pub struct Pusher<'a>(pub(crate) ThreadRef<'a>);

impl Pusher<'_> {
    #[inline]
    pub fn push<V: Pushable>(self, value: &V) {
        value.push(self);
    }
}

/// A trait for values that can be pushed onto the stack.
pub trait Pushable {
    fn push(&self, pusher: Pusher);
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
    type Err = ParseFloatError;

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

impl Pushable for LuaNumber {
    #[inline]
    fn push(&self, mut pusher: Pusher) {
        unsafe { sys::lua_pushnumber(pusher.0.as_raw().as_mut(), self.value) }
    }
}

macro_rules! lua_number_pushable_impl {
    ($type:ty) => {
        impl Pushable for $type {
            #[inline]
            fn push(&self, pusher: Pusher) {
                LuaNumber::from(*self).push(pusher)
            }
        }
    };
}

lua_number_pushable_impl!(f32);
lua_number_pushable_impl!(f64);

/// The Lua `nil` value.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct LuaNil;

impl fmt::Display for LuaNil {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("nil")
    }
}

impl Value for LuaNil {
    fn value_type() -> ValueType {
        ValueType::Nil
    }

    unsafe fn get_unchecked(thread: &mut Thread) -> LuaNil {
        sys::lua_pop(thread.as_raw().as_ptr(), 1);
        LuaNil
    }
}

impl Pushable for LuaNil {
    #[inline]
    fn push(&self, mut pusher: Pusher) {
        unsafe { sys::lua_pushnil(pusher.0.as_raw().as_ptr()) }
    }
}

#[repr(transparent)]
struct LuaStrRepr([u8]);

/// Representation of a borrowed Lua String.
/// It can be constructed safely from a `&[u8]` slice, or unsafely from a raw `*const u8`.
pub struct LuaStr {
    repr: LuaStrRepr,
}

impl LuaStr {
    /// Wraps a raw pointer with a safe Lua string wrapper.
    ///
    /// This function will wrap the provided `ptr` with a `LuaStr` wrapper, which
    /// allows inspection and interoperation of non-owned Lua strings. The total
    /// size of the raw Lua string must be smaller than `isize::MAX` **bytes**
    /// in memory due to calling the `slice::from_raw_parts` function.
    ///
    /// # Safety
    /// This method is unsafe for a number of reasons:
    ///
    /// * There is no guarantee to the validity of `ptr`.
    /// * The returned lifetime is not guaranteed to be the actual lifetime of
    ///   `ptr`.
    /// * It is not guaranteed that the memory pointed by `ptr` won't change
    ///   before the `LuaStr` has been destroyed.
    #[inline]
    pub unsafe fn from_ptr<'a>(ptr: *const u8, len: usize) -> &'a LuaStr {
        LuaStr::from_bytes(slice::from_raw_parts(ptr, len))
    }

    /// Creates a Lua string wrapper from a byte slice.
    #[inline(always)]
    pub fn from_bytes<B: AsRef<[u8]> + ?Sized>(bytes: &B) -> &LuaStr {
        LuaStr::from_bytes_impl(bytes.as_ref())
    }

    #[inline]
    fn from_bytes_impl(bytes: &[u8]) -> &LuaStr {
        unsafe { &*(bytes as *const [u8] as *const LuaStr) }
    }

    /// Converts this Lua string to a byte slice.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.repr.0
    }

    /// Yields a &str slice if the `LuaStr` contains valid UTF-8.
    ///
    /// If the contents of the `LuaStr` are valid UTF-8 data, this
    /// function will return the corresponding &str slice. Otherwise,
    /// it will return an error with details of where UTF-8 validation failed.
    ///
    /// > **Note**: This method is currently implemented to check for validity
    /// > after a constant-time cast, but it is planned to alter its definition
    /// > in the future to perform the length calculation in addition to the
    /// > UTF-8 check whenever this method is called.
    #[inline]
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.as_bytes())
    }

    /// Converts a `LuaStr` into a [`Cow`]`<`[`str`]`>`.
    ///
    /// If the contents of the `LuaStr` are valid UTF-8 data, this
    /// function will return a [`Cow`]`::`[`Borrowed`]`(`&str`)`
    /// with the corresponding &str slice. Otherwise, it will
    /// replace any invalid UTF-8 sequences with
    /// [`U+FFFD REPLACEMENT CHARACTER`][U+FFFD] and return a
    /// [`Cow`]`::`[`Owned`]`(`[`String`]`)` with the result.
    ///
    /// [`Cow`]: std::borrow::Cow
    /// [`Borrowed`]: std::borrow::Cow::Borrowed
    /// [`Owned`]: std::borrow::Cow::Owned
    /// [`String`]: std::string::String
    /// [U+FFFD]: std::char::REPLACEMENT_CHARACTER
    #[inline]
    pub fn to_string_lossy(&self) -> Cow<str> {
        String::from_utf8_lossy(self.as_bytes())
    }
}

impl fmt::Debug for LuaStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"")?;
        for byte in self
            .as_bytes()
            .iter()
            .flat_map(|&b| ascii::escape_default(b))
        {
            f.write_char(byte as char)?;
        }
        write!(f, "\"")
    }
}

impl fmt::Display for LuaStr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl AsRef<[u8]> for LuaStr {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.repr.0
    }
}

impl Default for &LuaStr {
    fn default() -> &'static LuaStr {
        const EMPTY: &[u8] = &[];
        LuaStr::from_bytes(EMPTY)
    }
}

impl PartialEq for LuaStr {
    #[inline]
    fn eq(&self, other: &LuaStr) -> bool {
        self.as_bytes().eq(other.as_bytes())
    }
}

impl Eq for LuaStr {}

impl PartialOrd for LuaStr {
    #[inline]
    fn partial_cmp(&self, other: &LuaStr) -> Option<Ordering> {
        self.as_bytes().partial_cmp(&other.as_bytes())
    }
}

impl Ord for LuaStr {
    #[inline]
    fn cmp(&self, other: &LuaStr) -> Ordering {
        self.as_bytes().cmp(&other.as_bytes())
    }
}

impl<'a> Pushable for &'a LuaStr {
    #[inline]
    fn push(&self, mut pusher: Pusher) {
        unsafe {
            sys::lua_pushlstring(
                pusher.0.as_raw().as_ptr(),
                self.repr.0.as_ptr() as *const libc::c_char,
                self.repr.0.len(),
            );
        }
    }
}

macro_rules! luastr_push_impl {
    ($type:ty) => {
        impl Pushable for $type {
            #[inline]
            fn push(&self, pusher: Pusher) {
                LuaStr::from_bytes(self).push(pusher)
            }
        }
    };
}

luastr_push_impl!(&'_ [u8]);
luastr_push_impl!(&'_ str);
luastr_push_impl!(String);
luastr_push_impl!(Vec<u8>);

/// `*mut T` lua wrapper type.
/// Like `*mut T`, `LightUserdata<T>` is invariant over `T`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LightUserdata<T: ?Sized> {
    ptr: *mut T,
}

impl<T> LightUserdata<T> {
    /// Creates a new `LightUserdata` that is `null`.
    #[inline]
    pub const fn null() -> LightUserdata<T> {
        LightUserdata {
            ptr: ptr::null_mut(),
        }
    }
}

impl<T: ?Sized> LightUserdata<T> {
    /// Creates a new `LightUserdata`.
    #[inline]
    pub const fn new(ptr: *mut T) -> LightUserdata<T> {
        LightUserdata { ptr }
    }

    /// Returns true if the pointer is null.
    ///
    /// Note that unsized types have many possible null pointers,
    /// as only the raw data pointer is considered, not their length, vtable, etc.
    /// Therefore, two pointers that are null may still not compare equal to each other.
    #[inline]
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    /// Acquires the underlying `*mut` pointer.
    #[inline]
    pub const fn as_ptr(&self) -> *mut T {
        self.ptr
    }

    /// Dereferences the content.
    ///
    /// The resulting lifetime is bound to self so this behaves "as if"
    /// it were actually an instance of T that is getting borrowed. If a longer
    /// (unbound) lifetime is needed, use `&*my_ptr.as_ptr()`.
    ///
    /// # Safety
    /// When calling this method, you have to ensure that if the pointer is
    /// non-NULL, then it is properly aligned, dereferencable (for the whole
    /// size of `T`) and points to an initialized instance of `T`. This applies
    /// even if the result of this method is unused!
    /// (The part about being initialized is not yet fully decided, but until
    /// it is the only safe approach is to ensure that they are indeed initialized.)
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub unsafe fn as_ref(&self) -> &T {
        &*self.as_ptr()
    }

    /// Mutably dereferences the content.
    ///
    /// The resulting lifetime is bound to self so this behaves "as if"
    /// it were actually an instance of T that is getting borrowed. If a longer
    /// (unbound) lifetime is needed, use `&mut *my_ptr.as_ptr()`.
    ///
    /// # Safety
    /// When calling this method, you have to ensure that if the pointer is
    /// non-NULL, then it is properly aligned, dereferencable (for the whole
    /// size of `T`) and points to an initialized instance of `T`. This applies
    /// even if the result of this method is unused!
    /// (The part about being initialized is not yet fully decided, but until
    /// it is the only safe approach is to ensure that they are indeed initialized.)
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        &mut *self.as_ptr()
    }

    /// Converts this `LuaUserData` to a [`NonNull`]
    ///
    /// [`NonNull`]: std::ptr::NonNull
    #[inline]
    pub fn into_non_null(self) -> Option<NonNull<T>> {
        NonNull::new(self.as_ptr())
    }
    /// Converts this `LuaUserData` to a [`NonNull`]
    ///
    /// # Safety
    /// The underlying pointer must be non-null.
    ///
    /// [`NonNull`]: std::ptr::NonNull
    #[inline]
    pub unsafe fn into_non_null_unchecked(self) -> NonNull<T> {
        NonNull::new_unchecked(self.as_ptr())
    }

    #[inline]
    pub const fn cast<U>(self) -> LightUserdata<U> {
        LightUserdata {
            ptr: self.as_ptr() as *mut U,
        }
    }
}

impl<T: ?Sized> Pointer for LightUserdata<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
    }
}

impl<T: RefUnwindSafe + ?Sized> UnwindSafe for LightUserdata<T> {}

impl<T: ?Sized> From<NonNull<T>> for LightUserdata<T> {
    #[inline]
    fn from(ptr: NonNull<T>) -> LightUserdata<T> {
        LightUserdata::new(ptr.as_ptr())
    }
}

mod private {
    use super::*;
    pub trait Sealed {}

    impl Sealed for LuaNumber {}
    impl Sealed for LuaNil {}
    impl Sealed for LuaStr {}
}
