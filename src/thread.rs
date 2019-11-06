use crate::*;

#[cfg(not(feature = "std"))]
use ::alloc::alloc::{self, Layout};
#[cfg(feature = "std")]
use std::alloc::{self, Layout};

use core::marker::PhantomData;
use core::mem::{self, ManuallyDrop};
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;
use core::slice;

/// Lua thread (state) wrapper.
#[derive(Debug)]
pub struct Thread {
    raw: NonNull<sys::lua_State>,
}

impl Thread {
    /// Creates a new thread running in a new, independent state.
    /// Can return an out of memory error if insufficient memory
    /// or a runtime error if there is a version mismatch.
    ///
    /// # Examples
    /// ```
    /// use pollua::thread::Thread;
    ///
    /// let thread = Thread::new().expect("Failed to create a Lua thread");
    /// ```
    #[inline]
    pub fn new() -> LuaResult<Thread> {
        unsafe { Thread::new_from::<()>(None, None, ptr::null_mut()) }
    }

    /// Creates a `Thread` from an allocator function and a panic handler.
    /// If `allocator` or `panic_handler` is set to `None`,
    /// `Thread` will use a default allocator or panic handler respectively.
    /// `userdata` is a (nullable) raw pointer passed to the allocator function,
    /// if `allocator` is `None` then this parameter is ignored.
    ///
    /// # Safety
    /// Behavior is undefined if `allocator` or `panic_handler` are invalid.
    #[inline(always)]
    pub unsafe fn new_from<T>(
        allocator: sys::lua_Alloc,
        panic_handler: sys::lua_CFunction,
        userdata: *mut T,
    ) -> LuaResult<Thread> {
        Thread::new_from_impl(allocator, panic_handler, userdata as *mut _)
    }

    /// Checks whether the core running the call, the core that created the Lua state,
    /// and the code making the call are all using the same version of Lua.
    /// Returns [`Error::Runtime`] if there is a version mismatch.
    ///
    /// [`Error::Runtime`]: struct.Thread.html#method.from_raw
    unsafe fn check_version(&mut self) -> LuaResult<()> {
        unsafe extern "C" fn check(l: *mut sys::lua_State) -> libc::c_int {
            sys::luaL_checkversion(l);
            0
        }

        // If luaL_checkversion failed, pcall will return an error
        sys::lua_pushcfunction(self.raw.as_ptr(), Some(check));
        self.get_error(sys::lua_pcall(self.raw.as_ptr(), 0, 0, 0))
    }

    /// Returns the error for the given `code`.
    /// If `code` is not `LUA_OK` then the object at stack index -1 is used as the error message.
    pub fn get_error(&mut self, code: libc::c_int) -> LuaResult<()> {
        if code == sys::LUA_OK {
            Ok(())
        } else {
            Err(Error {
                kind: match code {
                    sys::LUA_ERRRUN => ErrorKind::Runtime,
                    sys::LUA_ERRSYNTAX => ErrorKind::Syntax,
                    sys::LUA_ERRMEM => ErrorKind::OutOfMemory,
                    sys::LUA_ERRERR => ErrorKind::MessageHandler,
                    sys::LUA_ERRGCMM => ErrorKind::GarbageCollection,
                    sys::LUA_ERRFILE | _ => ErrorKind::Io,
                },
                msg: unsafe {
                    // check if there is a value at stack index -1
                    if sys::lua_isnone(self.as_raw().as_ptr(), -1) == 0 {
                        let mut len = 0usize;
                        // get the error object as a c string
                        let s = sys::luaL_tolstring(self.as_raw().as_ptr(), -1, &mut len as *mut _);
                        // luaL_tolstring also pushes its result to the stack, so we have to pop it.
                        sys::lua_pop(self.as_raw().as_ptr(), -1);
                        if s.is_null() {
                            None
                        } else {
                            // s is garanteed to be a valid c string at this point.
                            let buf = slice::from_raw_parts(s as *const u8, len);
                            Some(String::from_utf8_lossy(buf).into_owned())
                        }
                    } else {
                        None
                    }
                },
            })
        }
    }

    /// Returns the Lua version number.
    ///
    /// # Examples
    /// ```
    /// use pollua::thread::Thread;
    ///
    /// let thread = Thread::new().expect("Failed to create Lua thread");
    /// let thread_version = thread.version();
    ///
    /// assert_eq!(thread_version, pollua::lua_version());
    /// ```
    #[inline]
    pub fn version(&self) -> sys::lua_Number {
        unsafe { *sys::lua_version(self.raw.as_ptr()) }
    }

    /// Returns a raw pointer the wrapped `lua_State`.
    ///
    /// It is up to the caller to ensure that the object is still alive when accessing it through
    /// the pointer.
    ///
    /// # Examples
    /// ```
    /// use pollua::thread::Thread;
    ///
    /// let thread = Thread::new();
    /// ```
    ///
    #[inline]
    pub fn as_raw(&mut self) -> NonNull<sys::lua_State> {
        self.raw
    }

    /// Consumes the `Thread`, returning the wrapped raw pointer.
    ///
    /// # Examples
    /// Converting the raw pointer back into a `Thread` with [`Thread::from_raw`]:
    /// ```
    /// use pollua::thread::Thread;
    ///
    /// let thread = Thread::new().expect("Failed to create Lua thread");
    /// let ptr = thread.into_raw();
    ///
    /// let x = unsafe { Thread::from_raw(ptr) };
    /// ```
    ///
    /// [`Thread::from_raw`]: struct.Thread.html#method.from_raw
    #[inline]
    pub fn into_raw(self) -> NonNull<sys::lua_State> {
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
    pub unsafe fn from_raw(raw: NonNull<sys::lua_State>) -> Thread {
        Thread { raw }
    }

    /// Creates a `Thread` reference (of type [`ThreadRef`]) from a `lua_State` pointer.
    ///
    /// # Safety
    /// Behavior is undefined if `raw` is invalid or muliple reference to this state co-exist.
    ///
    /// [`ThreadRef`]: struct.ThreadRef.html
    #[inline]
    pub unsafe fn ref_from_raw<'a>(raw: NonNull<sys::lua_State>) -> &'a mut Thread {
        &mut *raw.cast::<Thread>().as_ptr()
    }

    /// Loads a Lua chunk without running it. If there are no errors,
    /// `load_bytes` pushes the compiled chunk as a Lua function on top of the stack.
    /// Otherwise, it returns an error.
    #[inline(always)]
    pub fn load_bytes<B: AsRef<[u8]>>(
        &mut self,
        to_load: B,
        chunk_name: Option<&str>,
        mode: LoadingMode,
    ) -> LuaResult<()> {
        self.load_bytes_impl(to_load.as_ref(), chunk_name, mode)
    }
}

// Method impls
impl Thread {
    pub unsafe fn new_from_impl(
        allocator: sys::lua_Alloc,
        panic_handler: sys::lua_CFunction,
        userdata: *mut libc::c_void,
    ) -> LuaResult<Thread> {
        let mut thread = Thread {
            raw: NonNull::new(match allocator {
                Some(_) => sys::lua_newstate(allocator, userdata),
                None => sys::lua_newstate(Some(alloc_default), ptr::null_mut()),
            })
            .ok_or_else(|| Error::new(ErrorKind::OutOfMemory, None))?,
        };
        sys::lua_atpanic(
            thread.raw.as_ptr(),
            panic_handler.or(Some(at_panic_default)),
        );
        thread.check_version()?;
        Ok(thread)
    }

    fn load_bytes_impl(
        &mut self,
        buffer: &[u8],
        chunk_name: Option<&str>,
        mode: LoadingMode,
    ) -> LuaResult<()> {
        let mut name_buf = Vec::new();
        unsafe {
            let code = sys::luaL_loadbufferx(
                self.as_raw().as_ptr(),
                util::cstr_unchecked(Some(buffer)),
                buffer.len(),
                util::cstr_buf(chunk_name, &mut name_buf),
                util::cstr_unchecked(Some(match mode {
                    LoadingMode::Binary => "b\0",
                    LoadingMode::Text => "t\0",
                    LoadingMode::Both => "bt\0",
                })),
            );
            self.get_error(code)
        }
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe {
            sys::lua_close(self.raw.as_ptr());
        }
    }
}

/// Used by the [`Thread::load_bytes`] method, describes how the bytes should be interpreted.
///
/// [`Thread::load_bytes`]: struct.Thread.html#method.load_bytes
#[derive(Debug, Copy, Clone)]
pub enum LoadingMode {
    Binary,
    Text,
    Both,
}

/// A mutable reference to a [`Thread`].
///
/// [`Thread`]: struct.Thread.html
pub struct ThreadRef<'a> {
    // thread's destructor must never be called, as it would trigger a double-free.
    thread: ManuallyDrop<Thread>,
    _marker: PhantomData<&'a mut Thread>,
}

impl<'a> ThreadRef<'a> {
    /// Creates a `ThreadRef` from a raw `lua_State` pointer.
    ///
    /// # Safety
    /// Behavior is undefined if `raw` is invalid or multiple reference to this `lua_State` exists.
    #[inline]
    pub unsafe fn from_raw(raw: NonNull<sys::lua_State>) -> ThreadRef<'a> {
        ThreadRef {
            thread: ManuallyDrop::new(Thread::from_raw(raw)),
            _marker: PhantomData,
        }
    }

    /// Creates a `ThreadRef` from a [`Thread`] mutable reference.
    ///
    /// [`Thread`]: struct.Thread.html
    #[inline]
    pub fn from_ref(thread: &'a mut Thread) -> ThreadRef<'a> {
        unsafe { ThreadRef::from_raw(thread.as_raw()) }
    }
}

impl<'a> Deref for ThreadRef<'a> {
    type Target = Thread;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.thread
    }
}

impl<'a> DerefMut for ThreadRef<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.thread
    }
}

/// Default panic handler function.
unsafe extern "C" fn at_panic_default(thread: *mut sys::lua_State) -> libc::c_int {
    match Thread::ref_from_raw(NonNull::new_unchecked(thread)).get_error(sys::LUA_ERRRUN) {
        Ok(()) => 0,
        Err(Error { msg: None, .. }) => panic!("Lua panic: <no error message>"),
        Err(Error { msg: Some(m), .. }) => panic!("Lua panic: {}", m),
    }
}

/// Default allocation function.
/// Uses the liballoc functions instead of the one from libc.
unsafe extern "C" fn alloc_default(
    _ud: *mut libc::c_void,
    ptr: *mut libc::c_void,
    osize: usize,
    nsize: usize,
) -> *mut libc::c_void {
    if nsize == 0 {
        alloc::dealloc(ptr as *mut u8, Layout::from_size_align_unchecked(osize, 1));
        ptr::null_mut()
    } else if ptr.is_null() {
        alloc::alloc(Layout::from_size_align_unchecked(nsize, 1)) as *mut _
    } else {
        alloc::realloc(
            ptr as *mut u8,
            Layout::from_size_align_unchecked(osize, 1),
            nsize,
        ) as *mut _
    }
}
