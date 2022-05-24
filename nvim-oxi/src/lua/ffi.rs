use libc::{c_char, c_int};

use super::lua_State;

// Pseudo-indices.
pub(crate) const LUA_REGISTRYINDEX: c_int = -10000;
pub(crate) const LUA_ENVIRONINDEX: c_int = -10001;
pub(crate) const LUA_GLOBALSINDEX: c_int = -10002;

pub(crate) const fn lua_upvalueindex(i: c_int) -> c_int {
    LUA_GLOBALSINDEX - i
}

// https://www.lua.org/manual/5.1/manual.html#lua_CFunction
pub(crate) type lua_CFunction =
    unsafe extern "C" fn(L: *mut lua_State) -> c_int;

extern "C" {
    // https://www.lua.org/manual/5.1/manual.html#lua_call
    pub(crate) fn lua_call(L: *mut lua_State, nargs: c_int, nresults: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_call
    pub(crate) fn lua_getfield(
        L: *mut lua_State,
        index: c_int,
        k: *const c_char,
    );

    // https://www.lua.org/manual/5.1/manual.html#lua_pushcclosure
    pub(crate) fn lua_pushcclosure(
        L: *mut lua_State,
        r#fn: lua_CFunction,
        n: c_int,
    );

    // https://www.lua.org/manual/5.1/manual.html#lua_pushstring
    pub(crate) fn lua_pushstring(L: *mut lua_State, s: *const c_char);

    // https://www.lua.org/manual/5.1/manual.html#lua_rawseti
    pub(crate) fn lua_rawseti(L: *mut lua_State, index: c_int, n: c_int);
}

// https://www.lua.org/manual/5.1/manual.html#lua_getglobal
#[allow(non_snake_case)]
#[inline(always)]
pub(crate) unsafe fn lua_getglobal(L: *mut lua_State, name: *const c_char) {
    lua_getfield(L, LUA_GLOBALSINDEX, name)
}

// https://www.lua.org/manual/5.1/manual.html#lua_pushcfunction
#[allow(non_snake_case)]
#[inline(always)]
pub(crate) unsafe fn lua_pushcfunction(
    L: *mut lua_State,
    r#fn: lua_CFunction,
) {
    lua_pushcclosure(L, r#fn, 0)
}
