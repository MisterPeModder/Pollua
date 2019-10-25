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

//! lua-sys - Raw ffi bindings for Lua 5.3

#![no_std]
#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::missing_safety_doc)]

extern crate libc;
#[cfg(feature = "va-list")]
extern crate va_list;

mod lua;

pub use lua::*;
