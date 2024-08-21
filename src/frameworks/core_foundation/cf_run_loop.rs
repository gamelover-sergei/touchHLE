/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CFRunLoop`.
//!
//! This is not even toll-free bridged to `NSRunLoop` in Apple's implementation,
//! but here it is the same type.

use crate::abi::GuestFunction;
use crate::dyld::{export_c_func, ConstantExports, FunctionExports, HostConstant};
use crate::frameworks::core_foundation::cf_allocator::CFAllocatorRef;
use crate::frameworks::core_foundation::cf_string::CFStringRef;
use crate::frameworks::core_foundation::time::{CFAbsoluteTime, CFTimeInterval};
use crate::frameworks::core_foundation::{CFIndex, CFTypeRef};
use crate::frameworks::foundation::ns_run_loop::run_run_loop_single_iteration;
use crate::frameworks::foundation::ns_string;
use crate::mem::{ConstVoidPtr, MutVoidPtr, Ptr};
use crate::objc::{id, msg, msg_class, nil, Class};
use crate::Environment;

pub type CFRunLoopRef = super::CFTypeRef;
pub type CFRunLoopMode = super::cf_string::CFStringRef;

pub type CFRunLoopTimerRef = super::CFTypeRef;
pub type CFOptionFlags = u32;

fn CFRunLoopGetCurrent(env: &mut Environment) -> CFRunLoopRef {
    msg_class![env; NSRunLoop currentRunLoop]
}

pub fn CFRunLoopGetMain(env: &mut Environment) -> CFRunLoopRef {
    msg_class![env; NSRunLoop mainRunLoop]
}

// CFRunLoopTimerRef CFRunLoopTimerCreate(
// CFAllocatorRef allocator, CFAbsoluteTime fireDate, CFTimeInterval interval,
// CFOptionFlags flags, CFIndex order, CFRunLoopTimerCallBack callout,
// CFRunLoopTimerContext *context)

// typedef void (*CFRunLoopTimerCallBack)(CFRunLoopTimerRef timer, void *info)
fn CFRunLoopTimerCreate(
    env: &mut Environment,
    _allocator: CFAllocatorRef,
    _fire_date: CFAbsoluteTime,
    interval: CFTimeInterval,
    flags: CFOptionFlags,
    order: CFIndex,
    callout: GuestFunction,
    context: MutVoidPtr,
) -> CFRunLoopTimerRef {
    assert_eq!(flags, 0);
    assert_eq!(order, 0);
    // assert!(context.is_null());

    let fake_target: id = msg_class![env; FakeCFTimerTarget alloc];
    let fake_target: id = msg![env; fake_target initWithCallout:callout context:context];

    let selector = env.objc.lookup_selector("timerFireMethod:").unwrap();

    let repeats = interval > 0.0;
    msg_class![env; NSTimer timerWithTimeInterval:interval
                                           target:fake_target
                                         selector:selector
                                         userInfo:nil
                                          repeats:repeats]
}

fn CFRunLoopAddTimer(
    env: &mut Environment,
    rl: CFRunLoopRef,
    timer: CFRunLoopTimerRef,
    mode: CFRunLoopMode,
) {
    let rl_class: Class = msg![env; rl class];
    assert_eq!(
        rl_class,
        env.objc.get_known_class("NSRunLoop", &mut env.mem)
    );

    let timer_class: Class = msg![env; timer class];
    assert_eq!(
        timer_class,
        env.objc.get_known_class("NSTimer", &mut env.mem)
    );

    () = msg![env; rl addTimer:timer forMode:mode];
}

fn CFRunLoopTimerInvalidate(env: &mut Environment, timer: CFRunLoopTimerRef) {
    let timer_class: Class = msg![env; timer class];
    // assert_eq!

        timer_class,
        (env.objc.get_known_class)("NSTimer", &mut env.mem)
    };

    () = msg![env; timer invalidate];
}

fn CFRunLoopRunInMode(
    env: &mut Environment,
    mode: CFRunLoopMode,
    seconds: CFTimeInterval,
    _return_something: bool,
) -> i32 {
    let default_mode = ns_string::get_static_str(env, kCFRunLoopDefaultMode);
    let common_modes = ns_string::get_static_str(env, kCFRunLoopCommonModes);
    // TODO: handle other modes
    assert!(
        msg![env; mode isEqualToString:default_mode]
            || msg![env; mode isEqualToString:common_modes]
    );
    // assert!(seconds <= 0.001);
    // TODO: we're currently supporting only the main run loop
    log_dbg!(
        "TODO: properly implement CFRunLoopRunInMode [current thread {}], running a single iteration of the main run loop",
        env.current_thread
    );
    let main_run_loop = CFRunLoopGetMain(env);
    run_run_loop_single_iteration(env, main_run_loop);
    1 // kCFRunLoopRunFinished
}

fn CFPreferencesCopyAppValue(env: &mut Environment, key: CFStringRef, appID: CFStringRef) -> CFTypeRef {
    Ptr::null()
}

fn CFPreferencesSetAppValue(env: &mut Environment, key: CFStringRef, value: CFTypeRef, appID: CFStringRef) {

}

fn CFPreferencesAppSynchronize(env: &mut Environment, appID: CFStringRef) -> bool {
    false
}

const kCFNumberSInt32Type: CFIndex = 3;

fn CFNumberCreate(
    env: &mut Environment, allocator: CFAllocatorRef, type_: CFIndex, value_ptr: ConstVoidPtr
) -> CFTypeRef {
    log!("CFNumberCreate type {}", type_);
    assert_eq!(type_, kCFNumberSInt32Type);
    let val: i32 = env.mem.read(value_ptr.cast());
    msg_class![env; NSNumber numberWithInt:val]
}

pub const kCFRunLoopCommonModes: &str = "kCFRunLoopCommonModes";
pub const kCFRunLoopDefaultMode: &str = "kCFRunLoopDefaultMode";
pub const kCFBundleVersionKey: &str = "CFBundleVersion";

pub const CONSTANTS: ConstantExports = &[
    (
        "_kCFRunLoopCommonModes",
        HostConstant::NSString(kCFRunLoopCommonModes),
    ),
    (
        "_kCFRunLoopDefaultMode",
        HostConstant::NSString(kCFRunLoopDefaultMode),
    ),
    (
        "_kCFBundleVersionKey",
        HostConstant::NSString(kCFBundleVersionKey),
    ),
];

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CFRunLoopGetCurrent()),
    export_c_func!(CFRunLoopGetMain()),
    export_c_func!(CFRunLoopTimerCreate(_, _, _, _, _, _, _)),
    export_c_func!(CFRunLoopTimerInvalidate(_)),
    export_c_func!(CFRunLoopAddTimer(_, _, _)),
    export_c_func!(CFRunLoopRunInMode(_, _, _)),
    export_c_func!(CFPreferencesCopyAppValue(_, _)),
    export_c_func!(CFPreferencesSetAppValue(_, _, _)),
    export_c_func!(CFPreferencesAppSynchronize(_)),
    export_c_func!(CFNumberCreate(_, _, _)),
];
