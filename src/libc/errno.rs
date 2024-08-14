/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `errno.h`

use crate::dyld::FunctionExports;
use crate::export_c_func;
use crate::mem::{ConstPtr, MutPtr};
use crate::Environment;
use std::io::{Read, Write};
use sdl2::libc::FILE;

pub const EPERM: i32 = 1;
pub const ENOENT: i32 = 2;
pub const EDEADLK: i32 = 11;
pub const EBUSY: i32 = 16;
pub const EINVAL: i32 = 22;

#[derive(Default)]
pub struct State {
    errnos: std::collections::HashMap<crate::ThreadId, MutPtr<i32>>,
}
impl State {
    fn errno_for_thread(
        &mut self,
        mem: &mut crate::mem::Mem,
        thread: crate::ThreadId,
    ) -> MutPtr<i32> {
        *self.errnos.entry(thread).or_insert_with(|| {
            log!(
                "TODO: errno accessed on thread {} (will always be 0)",
                thread
            );
            mem.alloc_and_write(0i32)
        })
    }
}

fn __error(env: &mut Environment) -> MutPtr<i32> {
    env.libc_state
        .errno
        .errno_for_thread(&mut env.mem, env.current_thread)
}

fn perror(env: &mut Environment, s: ConstPtr<u8>) {
    // TODO: errno mapping
    let errno_msg = "<TODO: errno>\n";
    let msg = if !s.is_null() {
        if let Ok(str) = env.mem.cstr_at_utf8(s) {
            format!("{}: {}", str, errno_msg)
        } else {
            errno_msg.to_string()
        }
    } else {
        errno_msg.to_string()
    };
    let _ = std::io::stderr().write_all(msg.as_bytes());
}

fn strerror(env: &mut Environment, errnum: i32) -> MutPtr<u8> {
    let str = format!("ERROR {}", errnum);
    env.mem.alloc_and_write_cstr(str.as_bytes())
}

fn __assert_rtn(env: &mut Environment, s1: ConstPtr<u8>, s2: ConstPtr<u8>, i: i32, s3: ConstPtr<u8>) {
    let ss1 = env.mem.cstr_at_utf8(s1).unwrap();
    let ss2 = env.mem.cstr_at_utf8(s2).unwrap();
    let ss3 = env.mem.cstr_at_utf8(s3).unwrap();
    log!("ASSERTION FAILED: {}, {}, {}, {}", ss1, ss2, i, ss3);
    // todo!()
}

fn ferror(env: &mut Environment, file_ptr: MutPtr<FILE>) -> i32 {
    0
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(__error()),
    export_c_func!(perror(_)),
    export_c_func!(strerror(_)),
    export_c_func!(__assert_rtn(_, _, _, _)),
    export_c_func!(ferror(_))
];
