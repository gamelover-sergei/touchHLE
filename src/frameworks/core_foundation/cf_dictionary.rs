/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CFDictionary` and `CFMutableDictionary`.
//!
//! These are toll-free bridged to `NSDictionary` and `NSMutableDictionary` in
//! Apple's implementation. Here they are the same types.

use std::ffi::c_void;
use super::cf_allocator::{kCFAllocatorDefault, CFAllocatorRef};
use super::CFIndex;
use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::foundation::{ns_string, NSUInteger};
use crate::mem::{ConstPtr, ConstVoidPtr, MutPtr, MutVoidPtr};
use crate::objc::{id, msg, msg_class, nil};
use crate::Environment;
use crate::frameworks::core_foundation::cf_data::CFDataRef;
use crate::frameworks::core_foundation::cf_string::CFStringRef;

pub type CFDictionaryRef = super::CFTypeRef;
pub type CFMutableDictionaryRef = super::CFTypeRef;

pub type CFPropertyListRef = super::CFTypeRef;

fn CFDictionaryCreateMutable(
    env: &mut Environment,
    allocator: CFAllocatorRef,
    capacity: CFIndex,
    keyCallbacks: ConstVoidPtr, // TODO, should be `const CFDictionaryKeyCallBacks*`
    valueCallbacks: ConstVoidPtr, // TODO, should be `const CFDictionaryValueCallBacks*`
) -> CFMutableDictionaryRef {
    assert!(allocator == kCFAllocatorDefault); // unimplemented
    // assert!(capacity == 0); // TODO: fixed capacity support
    // assert!(keyCallbacks.is_null()); // TODO: support retaining etc
    // assert!(valueCallbacks.is_null()); // TODO: support retaining etc

    //let new: id = msg_class![env; _touchHLE_NSMutableDictionary_non_retaining alloc];
    let new: id = msg_class![env; _touchHLE_NSMutableDictionary alloc];
    msg![env; new init]
}

fn CFDictionaryAddValue(
    env: &mut Environment,
    dict: CFMutableDictionaryRef,
    key: ConstVoidPtr,
    value: ConstVoidPtr
) {
    let key: id = key.cast().cast_mut();
    let res: id = msg![env; dict valueForKey:key];
    log!("CFDictionaryAddValue dict {:?} k {:?} v {:?}; res {:?}", dict, key, value, res);
    if res == nil {
        let value: id = value.cast().cast_mut();
        msg![env; dict setValue:value forKey:key]
    }
}

fn CFDictionarySetValue(
    env: &mut Environment,
    dict: CFMutableDictionaryRef,
    key: ConstVoidPtr,
    value: ConstVoidPtr
) {
    log!("CFDictionarySetValue k {:?} v {:?}", key, value);
    let key: id = key.cast().cast_mut();
    let value: id = value.cast().cast_mut();
    msg![env; dict setValue:value forKey:key]
}

fn CFDictionaryRemoveValue(
    env: &mut Environment,
    dict: CFMutableDictionaryRef,
    key: ConstVoidPtr
) {
    let key: id = key.cast().cast_mut();
    log!("CFDictionaryRemoveValue dict {:?} key {:?}", dict, key);
    () = msg![env; dict removeObjectForKey:key];
}

fn CFDictionaryRemoveAllValues(
    env: &mut Environment,
    dict: CFMutableDictionaryRef
) {
    let keys_arr: id = msg![env; dict allKeys];
    let enumerator: id = msg![env; keys_arr objectEnumerator];
    let mut key: id;
    loop {
        key = msg![env; enumerator nextObject];
        if key == nil {
            break;
        }
        CFDictionaryRemoveValue(env, dict, key.cast().cast_const());
    }
}

fn CFDictionaryGetValue(
    env: &mut Environment,
    dict: CFMutableDictionaryRef,
    key: ConstVoidPtr
) -> ConstVoidPtr {
    let key: id = key.cast().cast_mut();
    let res: id = msg![env; dict valueForKey:key];
    res.cast().cast_const()
}

fn CFDictionaryGetCount(env: &mut Environment, dict: CFDictionaryRef) -> CFIndex {
    let count: NSUInteger = msg![env; dict count];
    log!("CFDictionaryGetCount dict {:?} {}", dict, count);
    count.try_into().unwrap()
}

// void CFDictionaryGetKeysAndValues(CFDictionaryRef theDict, const void **keys, const void **values)
fn CFDictionaryGetKeysAndValues(
    env: &mut Environment,
    dict: CFDictionaryRef,
    keys: ConstPtr<MutVoidPtr>,
    values: ConstPtr<MutVoidPtr>) {
    let mut key_ptr = keys.cast_mut();
    let mut val_ptr = values.cast_mut();
    let keys_arr: id = msg![env; dict allKeys];
    let enumerator: id = msg![env; keys_arr objectEnumerator];
    let mut key: id;
    let mut val: id;
    loop {
        key = msg![env; enumerator nextObject];
        if key == nil {
            break;
        }
        if !key_ptr.is_null() {
            env.mem.write(key_ptr, key.cast());
            key_ptr += 1;
        }
        if !val_ptr.is_null() {
            val = msg![env; dict valueForKey:key];
            log!("CFDictionaryGetKeysAndValues dict {:?} key {:?} val {:?}", dict, key, val);
            env.mem.write(val_ptr, val.cast());
            val_ptr += 1;
        }
    }
}

// // CFDataRef CFPropertyListCreateXMLData(CFAllocatorRef allocator, CFPropertyListRef propertyList);
// fn CFPropertyListCreateXMLData(
//     env: &mut Environment,
//     allocator: CFAllocatorRef,
//     property_list: CFPropertyListRef
// ) -> CFDataRef {
//     nil
// }
// 
// // CFPropertyListRef CFPropertyListCreateFromXMLData(CFAllocatorRef allocator, CFDataRef xmlData,
// // CFOptionFlags mutabilityOption, CFStringRef *errorString);
// 
// fn CFPropertyListCreateFromXMLData(
//     env: &mut Environment,
//     allocator: CFAllocatorRef,
//     xml_data: CFDataRef,
//     flags: u32,
//     error_string: MutPtr<CFStringRef>
// ) -> CFPropertyListRef {
//     let err = ns_string::get_static_str(env, "error");
//     env.mem.write(error_string, err);
//     nil
// }

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CFDictionaryCreateMutable(_, _, _, _)),
    export_c_func!(CFDictionaryAddValue(_, _, _)),
    export_c_func!(CFDictionarySetValue(_, _, _)),
    export_c_func!(CFDictionaryRemoveValue(_, _)),
    export_c_func!(CFDictionaryRemoveAllValues(_)),
    export_c_func!(CFDictionaryGetValue(_, _)),
    export_c_func!(CFDictionaryGetCount(_)),
    export_c_func!(CFDictionaryGetKeysAndValues(_, _, _)),
    // export_c_func!(CFPropertyListCreateXMLData(_, _)),
    // export_c_func!(CFPropertyListCreateFromXMLData(_, _, _, _)),
];
