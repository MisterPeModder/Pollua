use crate::{
    thread::{Thread, ThreadRef},
    value::{Pushable, Pusher, ValueType},
    LuaResult,
};
use std::{
    cell::UnsafeCell,
    iter::{DoubleEndedIterator, FusedIterator},
    ops::Index,
};

/// Used to call Lua functions.
/// Created by the [`caller_*`] methods on [`Thread`].
///
/// [`caller_*`]: struct.Thread.html#method.caller_global
/// [`Thread`]: struct.Thread.html
#[derive(Debug)]
pub struct Caller<'a> {
    thread: ThreadRef<'a>,
    /// Number of arguments pushed to the stack.
    nargs: libc::c_int,
}

impl<'a> Caller<'a> {
    pub(super) fn from_global(mut thread: ThreadRef<'a>, name: &[u8]) -> Option<Caller<'a>> {
        // check if _G[name] is a function
        if thread.push_global(name) != sys::LUA_TFUNCTION {
            unsafe { sys::lua_pop(thread.as_raw().as_ptr(), 1) };
            None
        } else {
            Some(Caller { thread, nargs: 0 })
        }
    }

    /// Creates a `Caller` with the top stack value as the function
    /// The function will be popped from the stack upon calling it or when the `Caller` is dropped.
    ///
    /// # Safety
    /// Behavior is undefined if the value at the top of the stack is not a function.
    #[inline]
    pub(crate) unsafe fn from_stack_unchecked(mut thread: ThreadRef) -> Caller {
        debug_assert_eq!(
            sys::lua_type(thread.as_raw().as_ptr(), -1),
            sys::LUA_TFUNCTION
        );
        Caller { thread, nargs: 0 }
    }

    #[inline]
    pub fn arg<A: Pushable>(mut self, arg: A) -> Caller<'a> {
        unsafe { arg.push(Pusher(ThreadRef::from_raw(self.thread.as_raw()))) }
        self.nargs += 1;
        self
    }

    /// Executes the call, consuming the `Caller`.
    pub fn call(mut self) -> LuaResult<ReturnValues<'a>> {
        unsafe {
            // stack top before function and args were pushed
            let top = sys::lua_gettop(self.thread.as_raw().as_ptr()) - self.nargs - 1;
            let status = sys::lua_pcall(
                self.thread.as_raw().as_ptr(),
                self.nargs,
                sys::LUA_MULTRET,
                0,
            );
            self.nargs = -1;
            let nresults = sys::lua_gettop(self.thread.as_raw().as_ptr()) - top;
            self.thread
                .get_error(status)
                .map(|_| ReturnValues::new(self, nresults))
        }
    }

    /// Executes the call unprotected, consuming the `Caller`.
    ///
    /// # Safety
    /// Any error inside the called function will be propagated upwards with a `longjmp`,
    /// destroying the stack frames and skipping all destructor calls.
    /// Prefer using [`call`] if you are not sure whether the function will throw errors.
    /// See [Lua error handling] for more details.
    ///
    /// [`call`]: #method.call
    /// [Lua error handling]: https://www.lua.org/manual/5.3/manual.html#4.6
    pub unsafe fn call_unprotected(mut self) -> ReturnValues<'a> {
        // stack top before function and args were pushed
        let top = sys::lua_gettop(self.thread.as_raw().as_ptr()) - self.nargs - 1;
        sys::lua_pcall(
            self.thread.as_raw().as_ptr(),
            self.nargs,
            sys::LUA_MULTRET,
            0,
        );
        self.nargs = -1;
        let nresults = sys::lua_gettop(self.thread.as_raw().as_ptr()) - top;
        ReturnValues::new(self, nresults)
    }

    /// Executes the call, consuming the `Caller`.
    /// The number of results is adjusted to `nresults`.
    pub fn calln(mut self, nresults: u32) -> LuaResult<ReturnValues<'a>> {
        unsafe {
            let status = sys::lua_pcall(
                self.thread.as_raw().as_ptr(),
                self.nargs,
                nresults as libc::c_int,
                0,
            );
            self.nargs = -1;
            self.thread
                .get_error(status)
                .map(|_| ReturnValues::new(self, nresults as libc::c_int))
        }
    }

    /// Executes the call unprotected, consuming the `Caller`.
    /// The number of results is adjusted to `nresults`.
    ///
    /// # Safety
    /// Any error inside the called function will be propagated upwards with a `longjmp`,
    /// destroying the stack frames and skipping all destructor calls.
    /// Prefer using [`call`] if you are not sure whether the function will throw errors.
    /// See [Lua error handling] for more details.
    ///
    /// [`call`]: #method.call
    /// [Lua error handling]: https://www.lua.org/manual/5.3/manual.html#4.6
    pub unsafe fn calln_unprotected(mut self, nresults: u32) -> ReturnValues<'a> {
        sys::lua_call(
            self.thread.as_raw().as_ptr(),
            self.nargs,
            nresults as libc::c_int,
        );
        self.nargs = -1;
        ReturnValues::new(self, nresults as libc::c_int)
    }
}

impl<'a> Drop for Caller<'a> {
    fn drop(&mut self) {
        // Pops all remaining pushed elements from the stack
        unsafe {
            sys::lua_pop(
                self.thread.as_raw().as_ptr(),
                (self.nargs + 1) as libc::c_int,
            )
        };
    }
}

/// Holds the values produced by the [`call*`] methods on [`Caller`].
///
/// [`call*`]: struct.Caller.html#method.call
/// [`Caller`]: struct.Caller.html
#[derive(Debug)]
pub struct ReturnValues<'a> {
    thread: UnsafeCell<ThreadRef<'a>>,
    nresults: libc::c_int,
}

impl<'a> ReturnValues<'a> {
    #[inline]
    fn new(mut caller: Caller, nresults: libc::c_int) -> ReturnValues {
        ReturnValues {
            thread: unsafe { UnsafeCell::new(ThreadRef::from_raw(caller.thread.as_raw())) },
            nresults,
        }
    }

    #[inline]
    fn thread(&mut self) -> &mut Thread {
        unsafe { &mut *self.thread.get() }
    }

    #[inline]
    fn thread_ptr(&self) -> *mut sys::lua_State {
        unsafe { &mut *self.thread.get() }.as_raw().as_ptr()
    }

    /// Returns the number of values returned by the call.
    #[inline]
    pub fn len(&self) -> usize {
        self.nresults as usize
    }

    /// Returns true if the call did not return any value.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.nresults == 0
    }

    /// Returns the type of return value at the given position of `None` if out of bounds.
    #[inline]
    pub fn get(&self, index: usize) -> Option<ValueType> {
        if index < self.nresults as usize {
            ValueType::from_code(unsafe {
                sys::lua_type(self.thread_ptr(), -self.nresults + (index as libc::c_int))
            })
        } else {
            None
        }
    }

    /// Returns an iterator over the return values.
    #[inline]
    pub fn iter<'b>(&'b self) -> Iter<'a, 'b> {
        Iter {
            values: &self,
            start: 0,
            end: self.nresults,
        }
    }
}

impl Index<usize> for ReturnValues<'_> {
    type Output = ValueType;

    /// Returns the type of return value at the given position.
    ///
    /// # Panics
    /// This panics if `index` is out of bounds,
    /// prefer using [`get`] if you are not sure whether the index is valid.
    ///
    /// [`get`]: struct.ReturnValues.html#method.get
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        if index < self.nresults as usize {
            ValueType::from_code_ref(unsafe {
                sys::lua_type(self.thread_ptr(), -self.nresults + (index as libc::c_int))
            })
            .expect("no return value found in stack.\nTHIS IS A BUG, please report.")
        } else {
            panic!(
                "return value out of bounds: \
                 the number of return values is {}, but the index is {}",
                self.nresults, index
            );
        }
    }
}

impl Drop for ReturnValues<'_> {
    fn drop(&mut self) {
        unsafe { sys::lua_pop(self.thread().as_raw().as_ptr(), self.nresults) }
    }
}

/// Immutable return values iterator.
/// This struct is created by [`iter`] method on [`ReturnValues`].
///
/// [`iter`]: struct.ReturnValues.html#iter
/// [`ReturnValues`]: struct.ReturnValues.html
pub struct Iter<'a, 'b> {
    values: &'b ReturnValues<'a>,
    start: libc::c_int,
    end: libc::c_int,
}

impl<'a, 'b> Iterator for Iter<'a, 'b> {
    type Item = ValueType;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.start += 1;
            Some(self.values[(self.start - 1) as usize])
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let s = self.len();
        (s, Some(s))
    }
}

impl ExactSizeIterator for Iter<'_, '_> {
    #[inline]
    fn len(&self) -> usize {
        (self.end - self.start) as usize
    }
}

impl DoubleEndedIterator for Iter<'_, '_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end > self.start {
            self.end -= 1;
            Some(self.values[self.end as usize])
        } else {
            None
        }
    }
}

impl FusedIterator for Iter<'_, '_> {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{value::LuaNil, ErrorKind};
    use std::mem;

    fn stack_top(thread: &mut Thread) -> libc::c_int {
        unsafe { sys::lua_gettop(thread.as_raw().as_ptr()) }
    }

    #[test]
    fn test_call_global_no_args() {
        const TEXT: &[u8] = b"lorem ipsum";

        unsafe extern "C" fn test_call(l: *mut sys::lua_State) -> libc::c_int {
            sys::lua_pushboolean(l, 1);
            sys::lua_pushinteger(l, 67);
            sys::lua_pushlstring(l, TEXT.as_ptr() as *const _, 11);
            3
        }

        let mut thread = Thread::new().unwrap();
        let top = stack_top(&mut thread);
        unsafe {
            sys::lua_register(
                thread.as_raw().as_ptr(),
                b"test_call\0".as_ptr() as *const _,
                Some(test_call),
            );
        }

        // Dropping the caller without calling should pop the stack.
        mem::drop(thread.caller_global("test_call").unwrap());
        assert_eq!(stack_top(&mut thread), top);

        unsafe {
            let return_values = thread
                .caller_global("test_call")
                .unwrap()
                .calln_unprotected(0);
            assert_eq!(return_values.get(0), None);
        }
        assert_eq!(stack_top(&mut thread), top);

        {
            let return_values = thread.caller_global("test_call").unwrap().call().unwrap();
            assert_eq!(return_values[0], ValueType::Boolean);
            assert_eq!(return_values[1], ValueType::Number);
            assert_eq!(return_values[2], ValueType::String);
            assert_eq!(return_values.get(3), None);
        }
        assert_eq!(stack_top(&mut thread), top);

        {
            let return_values = thread.caller_global("test_call").unwrap().calln(2).unwrap();
            assert_eq!(return_values.get(0), Some(ValueType::Boolean));
            assert_eq!(return_values.get(1), Some(ValueType::Number));
            assert_eq!(return_values.get(2), None);
        }
        assert_eq!(stack_top(&mut thread), top);

        unsafe {
            let return_values = thread
                .caller_global("test_call")
                .unwrap()
                .calln_unprotected(4);
            assert_eq!(return_values[0], ValueType::Boolean);
            assert_eq!(return_values[1], ValueType::Number);
            assert_eq!(return_values.get(2), Some(ValueType::String));
            assert_eq!(return_values.get(3), Some(ValueType::Nil));
            assert_eq!(return_values.get(4), None);
        }
        assert_eq!(stack_top(&mut thread), top);
    }

    #[test]
    fn test_call_iter() {
        const TEXT: &[u8] = b"This is a test";

        unsafe extern "C" fn test_call(l: *mut sys::lua_State) -> libc::c_int {
            sys::lua_pushcfunction(l, Some(test_call));
            sys::lua_pushboolean(l, 0);
            sys::lua_pushlightuserdata(l, TEXT.as_ptr() as *mut _);
            sys::lua_pushinteger(l, 67);
            sys::lua_pushlstring(l, TEXT.as_ptr() as *const _, 14);
            5
        }

        let mut thread = Thread::new().unwrap();
        let top = stack_top(&mut thread);
        unsafe {
            sys::lua_register(
                thread.as_raw().as_ptr(),
                b"test_call\0".as_ptr() as *const _,
                Some(test_call),
            );
        }

        unsafe {
            let values = thread
                .caller_global("test_call")
                .unwrap()
                .calln_unprotected(0);
            let mut iter = values.iter();
            assert_eq!(iter.len(), 0);
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next_back(), None);
        }
        assert_eq!(stack_top(&mut thread), top);

        {
            let values = thread.caller_global("test_call").unwrap().call().unwrap();
            let mut iter = values.iter();
            assert_eq!(iter.len(), 5);
            assert_eq!(iter.next(), Some(ValueType::Function));
            assert_eq!(iter.next(), Some(ValueType::Boolean));
            assert_eq!(iter.next_back(), Some(ValueType::String));
            assert_eq!(iter.next_back(), Some(ValueType::Number));
            assert_eq!(iter.len(), 1);
            assert_eq!(iter.next(), Some(ValueType::LightUserdata));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next_back(), None);
        }
        assert_eq!(stack_top(&mut thread), top);
    }

    #[test]
    fn test_call_error() {
        unsafe extern "C" fn test_call(l: *mut sys::lua_State) -> libc::c_int {
            sys::lua_error(l)
        }

        let mut thread = Thread::new().unwrap();
        unsafe {
            sys::lua_register(
                thread.as_raw().as_ptr(),
                b"test_call\0".as_ptr() as *const _,
                Some(test_call),
            );
        }

        let err = thread
            .caller_global("test_call")
            .unwrap()
            .call()
            .unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Runtime);
    }

    #[test]
    fn test_call_sum() {
        unsafe extern "C" fn test_sum(l: *mut sys::lua_State) -> libc::c_int {
            let mut sum = 0.0;
            let nargs = match sys::lua_gettop(l) {
                n if n == 0 => sys::lua_error(l),
                n => n,
            };
            for i in 1..=nargs {
                sum += sys::luaL_checknumber(l, i);
            }
            sys::lua_pushnumber(l, sum);
            1
        }

        let mut thread = Thread::new().unwrap();
        let top = stack_top(&mut thread);
        unsafe {
            sys::lua_register(
                thread.as_raw().as_ptr(),
                b"test_sum\0".as_ptr() as *const _,
                Some(test_sum),
            );
        }

        {
            let err = thread
                .caller_global("test_sum")
                .unwrap()
                .call()
                .unwrap_err();
            assert_eq!(err.kind(), ErrorKind::Runtime);
        }
        assert_eq!(stack_top(&mut thread), top);

        {
            let err = thread
                .caller_global("test_sum")
                .unwrap()
                .arg(42.0)
                .arg(LuaNil)
                .call()
                .unwrap_err();
            assert_eq!(err.kind(), ErrorKind::Runtime);
        }
        assert_eq!(stack_top(&mut thread), top);

        {
            let return_values = thread
                .caller_global("test_sum")
                .unwrap()
                .arg(1.0f64)
                .arg(2.0f64)
                .call()
                .unwrap();
            assert_eq!(return_values.get(0), Some(ValueType::Number));
            assert_eq!(return_values.get(1), None);
        }
        assert_eq!(stack_top(&mut thread), top);
        {
            let return_values = thread
                .caller_global("test_sum")
                .unwrap()
                .arg(-2.0f64)
                .arg("24")
                .arg(7.2f64)
                .call()
                .unwrap();
            assert_eq!(return_values.get(0), Some(ValueType::Number));
            assert_eq!(return_values.get(1), None);
        }
        assert_eq!(stack_top(&mut thread), top);
    }
}
