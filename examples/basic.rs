/*
 * Copyright (c) 2019 Yanis Guaye
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http: //www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

extern crate lua_sys;
extern crate pollua;

use lua_sys::*;

fn main() {
    unsafe {
        let state = pollua::State::default();
        luaL_openlibs(state.as_mut_ptr());
        println!("Lua version: {}", *lua_version(state.as_mut_ptr()));
    }
}
