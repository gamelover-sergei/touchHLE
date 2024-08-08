/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! The `NSScanner` class.

use crate::frameworks::foundation::ns_string::from_u16_vec;
use crate::frameworks::foundation::unichar;
use crate::mem::MutPtr;
use crate::objc::{autorelease, id, nil, objc_classes, ClassExports, HostObject, NSZonePtr};
use crate::{msg, msg_class};

use super::ns_string::to_rust_string;
use super::NSUInteger;

// TODO: Speed up by optimizing for internal subclasses
#[derive(Default, Clone)]
struct NSScannerHostObject {
    string: id,      // NSString, should always be immutable since it's copied
    len: NSUInteger, // Length is cached since it is immutable.
    pos: NSUInteger,
}
impl HostObject for NSScannerHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSScanner: NSObject

+(id)scannerWithString:(id)string {
    let new: id = msg![env; this alloc];
    let new = msg![env; new initWithString:string];
    autorelease(env, new)
}

+ (id)allocWithZone:(NSZonePtr)zone {
    // NSScanner might be subclassed by something which needs
    // allocWithZone: to have the normal behaviour. Unimplemented: call
    // superclass alloc then.
    assert!(this == env.objc.get_known_class("NSScanner", &mut env.mem));
    msg_class![env; _touchHLE_NSScanner allocWithZone:zone]
}

@end

// Our private subclass that is the single implementation of NSCharacterSet for
// the time being.
@implementation _touchHLE_NSScanner: NSCharacterSet

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(NSScannerHostObject::default());
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithString:(id)string {
    assert!(string != nil);
    let string: id = msg![env; string copy]; // Same behaviour as simulator
    let len: NSUInteger = msg![env; string length];
    *env.objc.borrow_mut(this) = NSScannerHostObject {
        string,
        len,
        pos: 0
    };
    this
}

- (id)isAtEnd {
    nil
}

- (id)scanUpToString:(id)intoString {
    nil
}

- (bool) scanUpToCharactersFromSet:(id)cset intoString:(MutPtr<id>)str {
    let NSScannerHostObject { string, len, mut pos } = env.objc.borrow::<NSScannerHostObject>(this).clone();
    if pos >= len {
        env.mem.write(str, nil);
        return false;
    }
    let first_scan: unichar = msg![env; string characterAtIndex:pos];
    if msg![env; cset characterIsMember:first_scan] {
        env.mem.write(str, nil);
        return false;
    }
    let mut chars = vec![first_scan];
    pos += 1;
    while pos < len {
        let curr = msg![env; string characterAtIndex:pos];
        if msg![env; cset characterIsMember:first_scan] {
            break
        }
        chars.push(curr);
        pos += 1;
    }
    let out = from_u16_vec(env, chars);
    autorelease(env, out);
    env.mem.write(str, out);

    *env.objc.borrow_mut::<NSScannerHostObject>(this) = NSScannerHostObject { string, len, pos };
    true
}

- (bool) scanUpToCharactersFromSet:(id)cset intoString:(MutPtr<id>)str {
    let NSScannerHostObject { string, len, mut pos } = env.objc.borrow::<NSScannerHostObject>(this).clone();
    log!("{:?} {} {} {}", str, to_rust_string(env, string), pos, len);

    if pos >= len {
        if !str.is_null() {
            env.mem.write(str, nil);
        }
        return false;
    }
    let first_scan: unichar = msg![env; string characterAtIndex:pos];
    if msg![env; cset characterIsMember:first_scan] {
        if !str.is_null() {
            env.mem.write(str, nil);
        }
        return false;
    }
    let mut chars = vec![first_scan];
    pos += 1;
    while pos < len {
        let curr = msg![env; string characterAtIndex:pos];
        if msg![env; cset characterIsMember:curr] {
            break
        }
        chars.push(curr);
        pos += 1;
    }
    
    if !str.is_null() {
        let out = from_u16_vec(env, chars);
        autorelease(env, out);
        env.mem.write(str, out);
    }

    *env.objc.borrow_mut::<NSScannerHostObject>(this) = NSScannerHostObject { string, len, pos };
    true
}

- (bool) scanCharactersFromSet:(id)cset intoString:(MutPtr<id>)str {
    let inv_cset: id = msg![env; cset invertedSet];
    msg![env; this scanUpToCharactersFromSet:inv_cset intoString:str]
}
@end
};