// Lua Auxiliary Library

use crate::*;
use core::fmt;
use core::mem;
use core::ptr;

// //////////////////////////////////////////// //
// Constants                                    //
// //////////////////////////////////////////// //

pub const LUA_ERRFILE: libc::c_int = LUA_ERRERR + 1;

cfg_if::cfg_if! {
    if #[cfg(LUA_VERSION = "5.3")] {
        pub const LUA_LOADED_TABLE: &str = "_LOADED";

        pub const LUA_PRELOAD_TABLE: &str = "_PRELOAD";
    }
}

pub const LUA_NOREF: libc::c_int = -2;
pub const LUA_REFNIL: libc::c_int = -1;

#[cfg(LUA_VERSION = "5.1")]
pub const LUA_FILEHANDLE: &str = "FILE*";

// //////////////////////////////////////////// //
// Types                                        //
// //////////////////////////////////////////// //

#[cfg_attr(LUA_VERSION = "5.2", repr(C), derive(Clone))]
#[cfg(LUA_VERSION = "5.2")]
pub struct luaL_Buffer {
    /// Buffer address
    pub b: *mut libc::c_char,
    /// Buffer size
    pub size: usize,
    /// number of characters in buffer
    pub n: usize,
    pub L: *mut lua_State,
    /// initial buffer
    pub initb: [libc::c_char; LUAL_BUFFERSIZE],
}

#[cfg_attr(not(LUA_VERSION = "5.2"), repr(C), derive(Clone))]
#[cfg(not(LUA_VERSION = "5.2"))]
pub struct luaL_Buffer {
    /// Current position in buffer
    pub p: *mut libc::c_char,
    /// Number of strings in the stack (level)
    pub lvl: libc::c_int,
    pub L: *mut lua_State,
    pub buffer: [libc::c_char; LUAL_BUFFERSIZE],
}

impl fmt::Debug for luaL_Buffer {
    #[cfg(LUA_VERSION = "5.2")]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let initb = &self.initb as *const _;
        fmt.debug_struct("luaL_Buffer")
            .field("b", &self.b)
            .field("size", &self.size)
            .field("n", &self.n)
            .field("L", &self.L)
            .field("initb", &initb)
            .finish()
    }

    #[cfg(not(LUA_VERSION = "5.2"))]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let buffer = &self.buffer as *const _;
        fmt.debug_struct("luaL_Buffer")
            .field("p", &self.p)
            .field("lvl", &self.lvl)
            .field("L", &self.L)
            .field("buffer", &buffer)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}

#[repr(C)]
#[derive(Debug, Clone)]
struct luaL_Stream {
    /// Stream (NULL for incompletely created streams)
    f: *mut libc::FILE,
    /// To close stream (NULL for closed streams)
    closef: lua_CFunction,
}

// //////////////////////////////////////////// //
// Functions                                    //
// //////////////////////////////////////////// //

extern "C" {
    pub fn luaL_addlstring(B: *mut luaL_Buffer, s: *const libc::c_char, l: usize);
    pub fn luaL_addstring(B: *mut luaL_Buffer, s: *const libc::c_char);
    pub fn luaL_addvalue(B: *mut luaL_Buffer);
    pub fn luaL_argerror(L: *mut lua_State, arg: libc::c_int, extramsg: *const libc::c_char) -> !;
    pub fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);

    // Introduced in Lua 5.2
    #[cfg(LUA_VERSION = "5.2")]
    pub fn luaL_buffinitsize(
        L: *mut lua_State,
        B: *mut luaL_Buffer,
        sz: usize,
    ) -> *mut libc::c_char;

    pub fn luaL_callmeta(
        L: *mut lua_State,
        obj: libc::c_int,
        e: *const libc::c_char,
    ) -> libc::c_int;
    pub fn luaL_checkany(L: *mut lua_State, arg: libc::c_int);
    pub fn luaL_checkinteger(L: *mut lua_State, arg: libc::c_int) -> lua_Integer;
    pub fn luaL_checklstring(
        L: *mut lua_State,
        arg: libc::c_int,
        l: *mut usize,
    ) -> *const libc::c_char;
    pub fn luaL_checknumber(L: *mut lua_State, arg: libc::c_int) -> lua_Number;
    pub fn luaL_checkoption(
        L: *mut lua_State,
        arg: libc::c_int,
        def: *const libc::c_char,
        lst: *const *const libc::c_char,
    ) -> libc::c_int;
    pub fn luaL_checkstack(L: *mut lua_State, sz: libc::c_int, msg: *const libc::c_char);
    pub fn luaL_checktype(L: *mut lua_State, arg: libc::c_int, t: libc::c_int);
    pub fn luaL_checkudata(
        L: *mut lua_State,
        ud: libc::c_int,
        tname: *const libc::c_char,
    ) -> *mut libc::c_void;

    // Deprecated in Lua 5.3
    #[cfg(any(
        all(LUA_VERSION = "5.2", not(LUA_VERSION = "5.3")),
        all(LUA_VERSION = "5.3", feature = "lua-compat")
    ))]
    pub fn luaL_checkunsigned(L: *mut lua_State, arg: libc::c_int) -> lua_Unsigned;

    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: usize);
    pub fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, ...) -> !;

    // Introduced in Lua 5.2
    #[cfg(LUA_VERSION = "5.2")]
    pub fn luaL_execresult(L: *mut lua_State, stat: libc::c_int) -> libc::c_int;

    // Introduced in Lua 5.2
    #[cfg(LUA_VERSION = "5.2")]
    pub fn luaL_fileresult(
        L: *mut lua_State,
        stat: libc::c_int,
        fname: *const libc::c_char,
    ) -> libc::c_int;

    pub fn luaL_getmetafield(
        L: *mut lua_State,
        obj: libc::c_int,
        e: *const libc::c_char,
    ) -> libc::c_int;

    // Deprecated in Lua 5.1
    #[cfg(any(
        all(LUA_VERSION = "5.0", not(LUA_VERSION = "5.1")),
        all(LUA_VERSION = "5.1", feature = "lua-compat")
    ))]
    pub fn luaL_getn(L: *mut lua_State, t: libc::c_int) -> libc::c_int;

    // Introduced in Lua 5.2
    #[cfg(LUA_VERSION = "5.2")]
    pub fn luaL_getsubtable(
        L: *mut lua_State,
        idx: libc::c_int,
        fname: *const libc::c_char,
    ) -> libc::c_int;

    pub fn luaL_gsub(
        L: *mut lua_State,
        s: *const libc::c_char,
        p: *const libc::c_char,
        r: *const libc::c_char,
    ) -> *const libc::c_char;

    // Introduced in Lua 5.2
    #[cfg(LUA_VERSION = "5.2")]
    pub fn luaL_len(L: *mut lua_State, idx: libc::c_int) -> lua_Integer;

    // Defined as a macro as of Lua 5.2
    #[cfg(not(LUA_VERSION = "5.2"))]
    pub fn luaL_loadbuffer(
        L: *mut lua_State,
        buff: *const libc::c_char,
        sz: usize,
        name: *const libc::c_char,
    ) -> libc::c_int;

    // Introduced in Lua 5.2
    #[cfg(LUA_VERSION = "5.2")]
    pub fn luaL_loadbufferx(
        L: *mut lua_State,
        buff: *const libc::c_char,
        sz: usize,
        name: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;

    // Defined as a macro as of Lua 5.2
    #[cfg(not(LUA_VERSION = "5.2"))]
    pub fn luaL_loadfile(L: *mut lua_State, filename: *const libc::c_char) -> libc::c_int;

    // Introduced in Lua 5.2
    #[cfg(LUA_VERSION = "5.2")]
    pub fn luaL_loadfilex(
        L: *mut lua_State,
        filename: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;

    pub fn luaL_loadstring(L: *mut lua_State, s: *const libc::c_char) -> libc::c_int;
    pub fn luaL_newmetatable(L: *mut lua_State, tname: *const libc::c_char) -> libc::c_int;

    // Introduced in Lua 5.1
    #[cfg(LUA_VERSION = "5.1")]
    pub fn luaL_newstate() -> *mut lua_State;

    pub fn luaL_optinteger(L: *mut lua_State, arg: libc::c_int, def: lua_Integer) -> lua_Integer;
    pub fn luaL_optlstring(
        L: *mut lua_State,
        arg: libc::c_int,
        def: *const libc::c_char,
        l: *mut usize,
    ) -> *const libc::c_char;
    pub fn luaL_optnumber(L: *mut lua_State, arg: libc::c_int, def: lua_Number) -> lua_Number;

    // Deprecated in Lua 5.3
    #[cfg_attr(
        any(
            all(LUA_VERSION = "5.2", not(LUA_VERSION = "5.3")),
            all(LUA_VERSION = "5.3", feature = "lua-compat")
        ),
        inline
    )]
    #[cfg(any(
        all(LUA_VERSION = "5.2", not(LUA_VERSION = "5.3")),
        all(LUA_VERSION = "5.3", feature = "lua-compat")
    ))]
    pub fn luaL_optunsigned(L: *mut lua_State, arg: libc::c_int, def: lua_Unsigned)
        -> lua_Unsigned;

    // Defined as a macro in Lua 5.2+
    #[cfg(not(LUA_VERSION = "5.2"))]
    pub fn luaL_prepbuffer(B: *mut luaL_Buffer) -> *mut libc::c_char;

    // Introduced in Lua 5.2
    #[cfg(LUA_VERSION = "5.2")]
    pub fn luaL_prepbuffsize(B: *mut luaL_Buffer, sz: usize) -> *mut libc::c_char;

    pub fn luaL_pushresult(B: *mut luaL_Buffer);

    // Introduced in Lua 5.2
    #[cfg(LUA_VERSION = "5.2")]
    pub fn luaL_pushresultsize(B: *mut luaL_Buffer, sz: usize);

    pub fn luaL_ref(L: *mut lua_State, t: libc::c_int) -> libc::c_int;

    #[cfg(any(
        all(LUA_VERSION = "5.1", not(LUA_VERSION = "5.2")),
        all(LUA_VERSION = "5.2", feature = "lua-compat")
    ))]
    pub fn luaL_register(
        L: *mut lua_State,
        libname: *const libc::c_char,
        l: *const luaL_Reg,
    ) -> libc::c_int;

    pub fn luaL_requiref(
        L: *mut lua_State,
        modname: *const libc::c_char,
        openf: lua_CFunction,
        glb: libc::c_int,
    );
    pub fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: libc::c_int);
    pub fn luaL_setmetatable(L: *mut lua_State, tname: *const libc::c_char);

    // Deprecated in Lua 5.1
    #[cfg(any(
        all(LUA_VERSION = "5.0", not(LUA_VERSION = "5.1")),
        all(LUA_VERSION = "5.1", feature = "lua-compat")
    ))]
    pub fn luaL_setn(L: *mut lua_State, t: libc::c_int, n: libc::c_int);

    pub fn luaL_testudata(
        L: *mut lua_State,
        ud: libc::c_int,
        tname: *const libc::c_char,
    ) -> *mut libc::c_void;

    #[cfg(LUA_VERSION = "5.2")]
    pub fn luaL_tolstring(
        L: *mut lua_State,
        idx: libc::c_int,
        len: *mut usize,
    ) -> *const libc::c_char;

    pub fn luaL_traceback(
        L: *mut lua_State,
        L1: *mut lua_State,
        msg: *const libc::c_char,
        level: libc::c_int,
    );

    // Removed in Lua 5.2
    #[cfg(all(LUA_VERSION = "5.1", not(LUA_VERSION = "5.2")))]
    pub fn luaL_typerror(L: *mut lua_State, narg: libc::c_int, tname: *const libc::c_char) -> !;

    pub fn luaL_unref(L: *mut lua_State, t: libc::c_int, ref_: libc::c_int);
    pub fn luaL_where(L: *mut lua_State, lvl: libc::c_int);
}

// //////////////////////////////////////////// //
// Macros (represented as inline functions)     //
// //////////////////////////////////////////// //

#[cfg_attr(LUA_VERSION = "5.2", inline)]
#[cfg(LUA_VERSION = "5.2")]
pub unsafe fn luaL_addchar(B: *mut luaL_Buffer, c: libc::c_char) {
    if (*B).n >= (*B).size {
        luaL_prepbuffsize(B, 1);
    }
    (*B).b.add((*B).n).write(c);
    (*B).n += 1;
}

#[cfg_attr(not(LUA_VERSION = "5.2"), inline)]
#[cfg(not(LUA_VERSION = "5.2"))]
pub unsafe fn luaL_addchar(B: *mut luaL_Buffer, c: libc::c_char) {
    if (*B).p as usize >= (&(*B).buffer as *const _ as usize) + LUAL_BUFFERSIZE as usize {
        luaL_prepbuffer(B);
    }
    (*B).p.write(c);
    (*B).p = (*B).p.add(1);
}

#[cfg_attr(LUA_VERSION = "5.2", inline)]
#[cfg(LUA_VERSION = "5.2")]
pub unsafe fn luaL_addsize(B: *mut luaL_Buffer, n: usize) {
    (*B).n += n;
}

#[cfg_attr(not(LUA_VERSION = "5.2"), inline)]
#[cfg(not(LUA_VERSION = "5.2"))]
pub unsafe fn luaL_addsize(B: *mut luaL_Buffer, n: usize) {
    (*B).p = (*B).p.add(n);
}

#[inline]
pub unsafe fn luaL_argcheck(
    L: *mut lua_State,
    cond: libc::c_int,
    arg: libc::c_int,
    extramsg: *const libc::c_char,
) {
    if cond == 0 {
        luaL_argerror(L, arg, extramsg);
    }
}

// Deprecated in Lua 5.3
#[cfg_attr(
    any(
        all(LUA_VERSION = "5.0", not(LUA_VERSION = "5.3")),
        all(LUA_VERSION = "5.3", feature = "lua-compat")
    ),
    inline
)]
#[cfg(any(
    all(LUA_VERSION = "5.0", not(LUA_VERSION = "5.3")),
    all(LUA_VERSION = "5.3", feature = "lua-compat")
))]
pub unsafe fn luaL_checkint(L: *mut lua_State, arg: libc::c_int) -> libc::c_int {
    luaL_checkinteger(L, arg) as libc::c_int
}

// Deprecated in Lua 5.3
#[cfg_attr(
    any(
        all(LUA_VERSION = "5.0", not(LUA_VERSION = "5.3")),
        all(LUA_VERSION = "5.3", feature = "lua-compat")
    ),
    inline
)]
#[cfg(any(
    all(LUA_VERSION = "5.0", not(LUA_VERSION = "5.3")),
    all(LUA_VERSION = "5.3", feature = "lua-compat")
))]
pub unsafe fn luaL_checklong(L: *mut lua_State, arg: libc::c_int) -> libc::c_long {
    luaL_checkinteger(L, arg) as libc::c_long
}

#[inline]
pub unsafe fn luaL_checkstring(L: *mut lua_State, arg: libc::c_int) {
    luaL_checklstring(L, arg, ptr::null_mut());
}

#[inline]
pub unsafe fn luaL_checkversion(L: *mut lua_State) {
    luaL_checkversion_(
        L,
        LUA_VERSION_NUM,
        mem::size_of::<lua_Integer>() * 16 + mem::size_of::<lua_Number>(),
    );
}

#[inline]
pub unsafe fn luaL_dofile(L: *mut lua_State, filename: *const libc::c_char) -> libc::c_int {
    (luaL_loadfile(L, filename) != 0 || lua_pcall(L, 0, LUA_MULTRET, 0) != 0) as libc::c_int
}

#[inline]
pub unsafe fn luaL_dostring(L: *mut lua_State, string: *const libc::c_char) -> libc::c_int {
    (luaL_loadstring(L, string) != 0 || lua_pcall(L, 0, LUA_MULTRET, 0) != 0) as libc::c_int
}

#[inline]
pub unsafe fn luaL_getmetatable(L: *mut lua_State, tname: *const libc::c_char) -> libc::c_int {
    lua_getfield(L, LUA_REGISTRYINDEX, tname)
}

// Deprecated in Lua 5.1
#[cfg(all(
    LUA_VERSION = "5.1",
    not(LUA_VERSION = "5.2"),
    not(feature = "lua-compat")
))]
pub unsafe fn luaL_getn(L: *mut lua_State, t: libc::c_int) -> libc::c_int {
    lua_objlen(L, t) as libc::c_int
}

// Defined as a macro in Lua 5.2+
#[cfg_attr(LUA_VERSION = "5.2", inline)]
#[cfg(LUA_VERSION = "5.2")]
pub unsafe fn luaL_loadbuffer(
    L: *mut lua_State,
    buff: *const libc::c_char,
    sz: usize,
    name: *const libc::c_char,
) -> libc::c_int {
    luaL_loadbufferx(L, buff, sz, name, ptr::null())
}

// Defined as a macro in Lua 5.2+
#[cfg_attr(LUA_VERSION = "5.2", inline)]
#[cfg(LUA_VERSION = "5.2")]
pub unsafe fn luaL_loadfile(L: *mut lua_State, filename: *const libc::c_char) -> libc::c_int {
    luaL_loadfilex(L, filename, ptr::null())
}

// Introduced in Lua 5.2
#[cfg_attr(LUA_VERSION = "5.2", inline)]
#[cfg(LUA_VERSION = "5.2")]
pub unsafe fn luaL_newlib(L: *mut lua_State, l: *const luaL_Reg) {
    luaL_checkversion(L);
    luaL_newlibtable(L, l);
    luaL_setfuncs(L, l, 0)
}

// Introduced in Lua 5.2
#[cfg_attr(LUA_VERSION = "5.2", inline)]
#[cfg(LUA_VERSION = "5.2")]
pub unsafe fn luaL_newlibtable(L: *mut lua_State, _l: *const luaL_Reg) {
    lua_createtable(
        L,
        0,
        (mem::size_of::<*const luaL_Reg>() / mem::size_of::<luaL_Reg>() - 1) as libc::c_int,
    );
}

// Introduced in Lua 5.3
#[cfg_attr(LUA_VERSION = "5.3", inline)]
#[cfg(LUA_VERSION = "5.3")]
pub unsafe fn luaL_opt<T, D: Into<T>>(
    L: *mut lua_State,
    func: unsafe extern "C" fn(*mut lua_State, libc::c_int) -> T,
    arg: libc::c_int,
    dflt: D,
) -> T {
    if lua_isnoneornil(L, arg) != 0 {
        dflt.into()
    } else {
        func(L, arg)
    }
}

#[inline]
pub unsafe fn luaL_optstring(
    L: *mut lua_State,
    arg: libc::c_int,
    d: *const libc::c_char,
) -> *const libc::c_char {
    luaL_optlstring(L, arg, d, ptr::null_mut())
}

// Deprecated in Lua 5.3
#[cfg_attr(
    any(
        all(LUA_VERSION = "5.0", not(LUA_VERSION = "5.3")),
        all(LUA_VERSION = "5.3", feature = "lua-compat")
    ),
    inline
)]
#[cfg(any(
    all(LUA_VERSION = "5.0", not(LUA_VERSION = "5.3")),
    all(LUA_VERSION = "5.3", feature = "lua-compat")
))]
pub unsafe fn luaL_optint(L: *mut lua_State, arg: libc::c_int, d: libc::c_int) -> libc::c_int {
    luaL_optinteger(L, arg, d as lua_Integer) as libc::c_int
}

// Deprecated in Lua 5.3
#[cfg_attr(
    any(
        all(LUA_VERSION = "5.0", not(LUA_VERSION = "5.3")),
        all(LUA_VERSION = "5.3", feature = "lua-compat")
    ),
    inline
)]
#[cfg(any(
    all(LUA_VERSION = "5.0", not(LUA_VERSION = "5.3")),
    all(LUA_VERSION = "5.3", feature = "lua-compat")
))]
pub unsafe fn luaL_optlong(L: *mut lua_State, arg: libc::c_int, d: libc::c_long) -> libc::c_long {
    luaL_optinteger(L, arg, d as lua_Integer) as libc::c_long
}

// Defined as a macro in Lua 5.2+
#[cfg_attr(LUA_VERSION = "5.2", inline)]
#[cfg(LUA_VERSION = "5.2")]
pub unsafe fn luaL_prepbuffer(B: *mut luaL_Buffer) -> *mut libc::c_char {
    luaL_prepbuffsize(B, LUAL_BUFFERSIZE)
}

// Deprecated in Lua 5.1
#[cfg(all(
    LUA_VERSION = "5.1",
    not(LUA_VERSION = "5.2"),
    not(feature = "lua-compat")
))]
pub unsafe fn luaL_setn(_L: *mut lua_State, _t: libc::c_int, _n: libc::c_int) {
    // no op
}

#[inline]
pub unsafe fn luaL_typename(L: *mut lua_State, index: libc::c_int) -> *const libc::c_char {
    lua_typename(L, lua_type(L, index))
}
