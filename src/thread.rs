use crate::*;

use core::mem;

/// Lua thread (state) wrapper.
#[derive(Debug)]
pub struct Thread {
    raw: *mut sys::lua_State,
}

impl Thread {
    /// Creates a new thread running in a new, independent state.
    ///
    /// # Examples
    /// ```
    /// use pollua::thread::Thread;
    ///
    /// let thread = Thread::new();
    /// ```
    #[inline]
    pub fn new() -> Thread {
        Thread {
            raw: unsafe { sys::luaL_newstate() },
        }
    }

    /// Returns the lya version number.
    ///
    /// # Examples
    /// ```
    /// use pollua::thread::Thread;
    ///
    /// let thread = Thread::new();
    /// let thread_version = thread.version();
    ///
    /// assert_eq!(thread_version, pollua::lua_version());
    /// ```
    #[inline]
    pub fn version(&self) -> sys::lua_Number {
        unsafe { *sys::lua_version(self.as_raw() as *mut _) }
    }

    /// Returns a raw pointer the wrapped `lua_State`.
    ///
    /// It is up to the caller to ensure that the object is still alive when accessing it through
    /// the pointer.
    ///
    /// The pointer may be [`null`] or be dangling in case the object has already been destroyed.
    ///
    /// # Examples
    /// ```
    /// use pollua::thread::Thread;
    ///
    /// let thread = Thread::new();
    ///
    /// unsafe {
    ///     println!("Lua ")
    /// }
    /// ```
    ///
    /// [`null`]: ::core::ptr::null
    #[inline]
    pub fn as_raw(&self) -> *const sys::lua_State {
        self.raw
    }

    /// Returns a mutable raw pointer the wrapped `lua_State`.
    #[inline]
    pub fn as_raw_mut(&self) -> *mut sys::lua_State {
        self.raw
    }

    /// Consumes the `Thread`, returning the wrapped raw pointer.
    ///
    /// # Examples
    /// Converting the raw pointer back into a `Thread` with [`Thread::from_raw`]:
    /// ```
    /// use pollua::thread::Thread;
    ///
    /// let thread = Thread::new();
    /// let ptr = thread.into_raw();
    ///
    /// let x = unsafe { Thread::from_raw(ptr) };
    /// ```
    ///
    /// [`Thread::from_raw`]: struct.Thread.html#method.from_raw
    #[inline]
    pub fn into_raw(self) -> *mut sys::lua_State {
        let raw = self.raw;
        mem::forget(self);
        raw
    }

    /// Constructs a `Thread` from a raw pointer.
    ///
    /// After calling this function, the raw pointer is owned by the resulting `Thread`.
    /// Specifically, the `Thread` destructor will call [`lua_close`] and free the lua state.
    ///
    /// # Safety
    /// Behavior is undefined if the pointer is invalid or already in use.
    ///
    /// [`lua_close`]: sys::lua_close
    #[inline]
    pub unsafe fn from_raw(raw: *mut sys::lua_State) -> Thread {
        Thread { raw }
    }

    pub fn load_bytes<B: AsRef<[u8]>>(
        &mut self,
        to_load: B,
        chunk_name: Option<&str>,
        mode: LoadingMode,
    ) -> LuaResult<()> {
        let buffer = to_load.as_ref();
        let mut name_buf = Vec::new();
        unsafe {
            Error::from_code(sys::luaL_loadbufferx(
                self.as_raw_mut(),
                crate::cstr_unchecked(Some(buffer)),
                buffer.len(),
                crate::cstr_buf(chunk_name, &mut name_buf),
                crate::cstr_unchecked(Some(match mode {
                    LoadingMode::Binary => "b\0",
                    LoadingMode::Text => "t\0",
                    LoadingMode::Both => "bt\0",
                })),
            ) as i32)
        }
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe {
            sys::lua_close(self.raw);
        }
    }
}

impl Default for Thread {
    #[inline]
    fn default() -> Thread {
        Thread::new()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum LoadingMode {
    Binary,
    Text,
    Both,
}
