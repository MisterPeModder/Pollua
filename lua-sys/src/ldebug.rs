use crate::*;

// //////////////////////////////////////////// //
// Lua types                                    //
// //////////////////////////////////////////// //

#[repr(C)]
pub struct lua_Debug {
    pub event: libc::c_int,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub currentline: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
    pub nups: libc::c_uchar,
    pub nparams: libc::c_uchar,
    pub isvararg: libc::c_char,
    pub istailcall: libc::c_char,
    pub short_src: [libc::c_char; LUA_IDSIZE],
    /* private part */
    _private: private::lua_Debug,
}

pub type lua_Hook = Option<unsafe extern "C" fn(L: *mut lua_State, ar: *mut lua_Debug)>;

#[allow(unused)]
mod private {
    use super::*;

    #[repr(C)]
    pub struct lua_Debug {
        i_ci: *mut CallInfo,
    }

    #[repr(C)]
    struct CallInfo {
        _private: [u8; 0],
    }
}

// //////////////////////////////////////////// //
// Lua Functions                                //
// //////////////////////////////////////////// //

extern "C" {
    pub fn lua_gethook(L: *mut lua_State) -> lua_Hook;
    pub fn lua_gethookcount(L: *mut lua_State) -> libc::c_int;
    pub fn lua_gethookmask(L: *mut lua_State) -> libc::c_int;
    pub fn lua_getinfo(
        L: *mut lua_State,
        what: *const libc::c_char,
        ar: *mut lua_Debug,
    ) -> libc::c_int;
    pub fn lua_getlocal(
        L: *mut lua_State,
        ar: *const lua_Debug,
        n: libc::c_int,
    ) -> *const libc::c_char;
    pub fn lua_getstack(L: *mut lua_State, level: libc::c_int, ar: *mut lua_Debug) -> libc::c_int;
    pub fn lua_getupvalue(
        L: *mut lua_State,
        funcindex: libc::c_int,
        n: libc::c_int,
    ) -> *const libc::c_char;
    pub fn lua_sethook(L: *mut lua_State, func: lua_Hook, mask: libc::c_int, count: libc::c_int);
    pub fn lua_setlocal(
        L: *mut lua_State,
        ar: *const lua_Debug,
        n: libc::c_int,
    ) -> *const libc::c_char;
    pub fn lua_setupvalue(
        L: *mut lua_State,
        funcindex: libc::c_int,
        n: libc::c_int,
    ) -> *const libc::c_char;
    pub fn lua_upvalueid(L: *mut lua_State, fidx: libc::c_int, n: libc::c_int)
        -> *mut libc::c_void;
    pub fn lua_upvaluejoin(
        L: *mut lua_State,
        fidx1: libc::c_int,
        n1: libc::c_int,
        fidx2: libc::c_int,
        n2: libc::c_int,
    );
}
