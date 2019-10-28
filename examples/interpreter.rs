extern crate pollua;

use pollua::sys;
use pollua::State;
use std::io;
use std::io::prelude::*;

fn init_state(state: &mut State) {
    unsafe {
        sys::luaL_checkversion(state.as_mut_ptr());
        sys::luaL_openlibs(state.as_mut_ptr());
    }
}

fn run_line(state: &mut State, line: &str) {
    unsafe {
        if validate(
            state,
            sys::luaL_loadbufferx(
                state.as_mut_ptr(),
                line.as_ptr() as *const _,
                line.len(),
                b"<stdin>\0".as_ptr() as *const _,
                b"t\0".as_ptr() as *const _,
            ),
        ) {
            validate(
                state,
                sys::lua_pcall(state.as_mut_ptr(), 0, sys::LUA_MULTRET, 0),
            );
        }
    }
}

fn validate(state: &mut State, code: libc::c_int) -> bool {
    if code == sys::LUA_OK {
        return true;
    } else if code == sys::LUA_ERRSYNTAX {
        print!("\u{001b}[31;1msyntax error");
    } else if code == sys::LUA_ERRMEM {
        print!("\u{001b}[31;1mout of memory");
    } else if code == sys::LUA_ERRGCMM {
        print!("\u{001b}[31;1merror while running gc metamethod");
    } else if code == sys::LUA_ERRRUN {
        print!("\u{001b}[31;1mruntime error");
    } else if code == sys::LUA_ERRERR {
        print!("\u{001b}[31;1merror while running message handler");
    }
    unsafe {
        let top = sys::lua_gettop(state.as_mut_ptr());
        if sys::lua_isnone(state.as_mut_ptr(), top) == 0 {
            let err = sys::lua_tostring(state.as_mut_ptr(), top);
            if !err.is_null() {
                print!(": {}", std::ffi::CStr::from_ptr(err).to_string_lossy());
            }
        }
    }
    println!("\u{001b}[0m");
    false
}

fn main() {
    let mut state = State::default();
    init_state(&mut state);
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut prev_line = String::new();

    println!("{} (Pollua/lua-sys)", sys::LUA_RELEASE);
    println!("Type \"exit\" to quit.\n");
    print!(">>> ");
    stdout.flush().unwrap();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let line = format!("{}{}", prev_line, line.trim());
        if line == "exit" {
            break;
        }
        if line.ends_with('\\') {
            prev_line = line;
            prev_line.truncate(prev_line.len() - 1);
            prev_line.push(' ');
            print!("... ");
            stdout.flush().unwrap();
            continue;
        }
        prev_line.clear();
        run_line(&mut state, &line);
        print!(">>> ");
        stdout.flush().unwrap();
    }
}
