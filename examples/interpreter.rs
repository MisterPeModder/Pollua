extern crate pollua;

use pollua::sys;
use pollua::thread::LoadingMode;
use pollua::Thread;
use std::io::{self, BufRead, Write};

fn main() {
    // Lua thread init
    let mut thread = Thread::new().expect("unable to crate Lua thread");
    unsafe { sys::luaL_openlibs(thread.as_raw().as_ptr()) };

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut prev_line = String::new();

    // display message and first prompt
    println!("{} (Pollua/lua-sys)", sys::LUA_RELEASE);
    println!("Type \"exit\" to quit.\n");
    print!(">>> ");
    stdout.flush().unwrap();

    for line in stdin.lock().lines().map(Result::unwrap) {
        let line = format!("{}{}", prev_line, line.trim());
        if line == "exit" {
            break;
        }

        // line continuation handling
        if line.ends_with('\\') {
            prev_line = line;
            prev_line.truncate(prev_line.len() - 1);
            prev_line.push(' ');
            print!("... ");
            stdout.flush().unwrap();
            continue;
        }
        prev_line.clear();

        // run the line and check for errors
        if let Err(e) = thread
            .caller_load(&line, Some("<stdin>"), LoadingMode::Text)
            .and_then(|c| c.call())
        {
            println!("\u{001b}[31;1m{}\u{001b}[0m", e);
        }

        // print the next prompt
        print!(">>> ");
        stdout.flush().unwrap();
    }
}
