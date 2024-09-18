/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSData` and `NSMutableData`.

use super::ns_string::to_rust_string;
use super::{NSRange, NSInteger, NSUInteger};
use crate::fs::GuestPath;
use crate::mem::{ConstPtr, ConstVoidPtr, MutPtr, MutVoidPtr, Ptr};
use crate::objc::{
    autorelease, id, msg, nil, objc_classes, release, retain, ClassExports, HostObject, NSZonePtr,
};
use crate::{msg_class, Environment};

struct NSDataHostObject {
    bytes: MutVoidPtr,
    freeWhenDone: MutVoidPtr,
    length: NSUInteger,
}
impl HostObject for NSDataHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

// NSData doesn't seem to be an abstract class?
@implementation NSData: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(NSDataHostObject {
        bytes: Ptr::null(),
        freeWhenDone: Ptr::null(),
        length: 0,
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

+ (id)data {
    nil
}

+ (id)dataWithBytesNoCopy:(MutVoidPtr)bytes
                   length:(NSUInteger)length {
    let new: id = msg![env; this alloc];
    let new: id = msg![env; new initWithBytesNoCopy:bytes length:length];
    autorelease(env, new)
}

+ (id)dataWithBytes:(MutVoidPtr)bytes
             length:(NSUInteger)length {
    let new: id = msg![env; this alloc];
    let new: id = msg![env; new initWithBytes:bytes length:length];
    autorelease(env, new)
}

+ (id)dataWithContentsOfFile:(id)path {
    let new: id = msg![env; this alloc];
    let new: id = msg![env; new initWithContentsOfFile:path];
    autorelease(env, new)
}

+ (id)dataWithContentsOfMappedFile:(id)path {
    log!("TODO [NSData dataWithContentsOfMappedFile:{:?}] uses [NSData dataWithContentsOfFile:] implementation instead of its own", path);
    let new: id = msg![env; this alloc];
    let new: id = msg![env; new initWithContentsOfFile:path];
    autorelease(env, new)
}

+ (id)dataWithContentsOfURL:(id)url {
    let new: id = msg![env; this alloc];
    let new: id = msg![env; new initWithContentsOfURL:url];
    autorelease(env, new)
}

+ (id)length {
    nil
}

+ (())dataWithBytesNoCopy:(NSInteger)copy length:(bool)_length freeWhenDone:(bool)_done {
    // TODO
}

+ (())dataWithContentsOfFile:(NSInteger)file options:(bool)_options error:(bool)_error {
    // TODO
}

// Calling the standard `init` is also allowed, in which case we just get data
// of size 0.

- (id)initWithBytesNoCopy:(NSUInteger)length
                     freeWhenDone:(MutVoidPtr)freeWhenDone {
    let host_object = env.objc.borrow_mut::<NSDataHostObject>(this);
    assert!(host_object.freeWhenDone.is_null() && host_object.length == 0);
    host_object.freeWhenDone = freeWhenDone;
    host_object.length = length;
    this
}

- (id)initWithBytesNoCopy:(MutVoidPtr)bytes
                   length:(NSUInteger)length {
    let host_object = env.objc.borrow_mut::<NSDataHostObject>(this);
    assert!(host_object.bytes.is_null() && host_object.length == 0);
    host_object.bytes = bytes;
    host_object.length = length;
    this
}

- (id)initWithBytes:(MutVoidPtr)bytes
              length:(NSUInteger)length {
    let host_object = env.objc.borrow_mut::<NSDataHostObject>(this);
    assert!(host_object.bytes.is_null() && host_object.length == 0);
    let alloc = env.mem.alloc(length);
    env.mem.memmove(alloc, bytes.cast_const(), length);
    host_object.bytes = alloc;
    host_object.length = length;
    this
}

- (id)initWithContentsOfURL:(id)url { // NSURL *
    let path: id = msg![env; url absoluteString];
    let path = to_rust_string(env, path);
    // TODO: file URL case
    assert!(path.starts_with("http"));
    log!("TODO: ignoring [(NSData*){:?} initWithContentsOfURL:{:?}]", this, path);
    // TODO: actually load data once we have proper network support
    nil
}

- (id)initWithContentsOfFile:(id)path {
    if path == nil {
        return nil
    }
    let path = to_rust_string(env, path);
    log_dbg!("[(NSData*){:?} initWithContentsOfFile:{:?}]", this, path);
    let Ok(bytes) = env.fs.read(GuestPath::new(&path)) else {
        release(env, this);
        return nil;
    };
    let size = bytes.len().try_into().unwrap();
    let alloc = env.mem.alloc(size);
    let slice = env.mem.bytes_at_mut(alloc.cast(), size);
    slice.copy_from_slice(&bytes);

    let host_object = env.objc.borrow_mut::<NSDataHostObject>(this);
    host_object.bytes = alloc;
    host_object.length = size;
    this
}

- (id)initWithContentsOfMappedFile:(id)path {
    // IMM?: This is ok, right?
    msg![env; this initWithContentsOfFile:path]
}

- (id)initWithData:(NSUInteger)_data {
    msg![env; this init]
}

- (())initWithBytesNoCopy:(NSInteger)copy length:(bool)_length freeWhenDone:(bool)_done {
    // TODO
}

- (bool)writeToFile:(id)path // NSString*
            options:(NSUInteger)_options_mask
              error:(MutPtr<id>)error { // NSError**
    let success: bool = msg![env; this writeToFile:path atomically:true];
    if !success && !error.is_null() {
        todo!(); // TODO: create an NSError if requested
    }
    success
}

// FIXME: writes should be atomic
- (bool)writeToFile:(id)path // NSString*
         atomically:(bool)_use_aux_file {
    let file = to_rust_string(env, path);
    log_dbg!("[(NSData*){:?} writeToFile:{:?} atomically:_]", this, file);
    let host_object = env.objc.borrow::<NSDataHostObject>(this);
    // Mem::bytes_at() panics when the pointer is NULL, but NSData's pointer can
    // be NULL if the length is 0.
    let slice = if host_object.length == 0 {
        &[]
    } else {
        env.mem.bytes_at(host_object.bytes.cast(), host_object.length)
    };
    env.fs.write(GuestPath::new(&file), slice).is_ok()
}

- (())dealloc {
    let &NSDataHostObject { bytes, .. } = env.objc.borrow(this);
    if !bytes.is_null() {
        env.mem.free(bytes);
    }
    env.objc.dealloc_object(this, &mut env.mem)
}

// NSCopying implementation
- (id)copyWithZone:(NSZonePtr)_zone {
    retain(env, this)
}

- (id)alloc {
    nil
}

- (id)mutableCopyWithZone:(NSZonePtr)_zone {
    let bytes: ConstVoidPtr = msg![env; this bytes];
    let length: NSUInteger = msg![env; this length];
    let new = msg_class![env; NSMutableData alloc];
    msg![env; new initWithBytes:(bytes.cast_mut()) length:length]
}

- (ConstVoidPtr)bytes {
    env.objc.borrow::<NSDataHostObject>(this).bytes.cast_const()
}
- (NSUInteger)length {
    env.objc.borrow::<NSDataHostObject>(this).length
}

- (ConstVoidPtr)freeWhenDone {
    env.objc.borrow::<NSDataHostObject>(this).freeWhenDone.cast_const()
}

- (bool)isEqualToData:(id)other {
    // FIXME: Avoid allocation
    let a = to_rust_slice(env, this).to_owned();
    let b = to_rust_slice(env, other);
    a == b
}

- (())getBytes:(MutPtr<u8>)buffer length:(NSUInteger)length {
    let length = length.min(env.objc.borrow::<NSDataHostObject>(this).length);
    let range = NSRange { location: 0, length };
    msg![env; this getBytes:buffer range:range]
}

- (())getBytes:(MutPtr<u8>)buffer range:(NSRange)range {
    if range.length == 0 {
        return;
    }
    let &NSDataHostObject { bytes, length, .. } = env.objc.borrow(this);
    // TODO: throw NSRangeException if out-of-range instead of panic?
    // assert!(range.location < length && range.location + range.length <= length);
    env.mem.memmove(
        buffer.cast(),
        bytes.cast_const() + range.location,
        range.length,
    );
}

- (())getBytes:(MutPtr<u8>)buffer {
    let &NSDataHostObject { bytes, length, .. } = env.objc.borrow(this);
    env.mem.memmove(
        buffer.cast(),
        bytes.cast_const(),
        length,
    );
}

@end

@implementation NSMutableData: NSData

+ (id)bytes {
    nil
}

+ (id)mutableBytes {
    nil
}

+ (id)length {
    nil
}

+ (id)dataWithCapacity:(NSUInteger)_capacity {
    msg![env; this init]
}

+ (id)dataWithData:(NSUInteger)_data {
    msg![env; this init]
}

+ (id)dataWithLength:(NSUInteger)_length {
    msg![env; this init]
}

+ (())appendBytes:(NSInteger)bytes length:(bool)_length {
    // TODO
}

- (id)initWithCapacity:(NSUInteger)_capacity {
    msg![env; this init]
}

- (id)initWithLength:(NSUInteger)_length {
    msg![env; this init]
}

- (id)appendData {
    nil
}

- (())setLength:(bool)length {
    log!("TODO: setLength:{}", length);
}

- (id)copyWithZone:(NSZonePtr)_zone {
    let bytes: ConstVoidPtr = msg![env; this bytes];
    let length: NSUInteger = msg![env; this length];
    let new = msg_class![env; NSData alloc];
    msg![env; new initWithBytes:bytes length:length]
}

- (())increaseLengthBy:(NSUInteger)add_len {
    let &NSDataHostObject { bytes, length, .. } = env.objc.borrow(this);
    let new_len = length + add_len;
    let new_bytes = env.mem.realloc(bytes, new_len);
    let host = env.objc.borrow_mut::<NSDataHostObject>(this);
    host.length = new_len;
    host.bytes = new_bytes;
    log_dbg!("increaseLengthBy bytes {:?}, new_bytes {:?}; length {}, new_len {}", bytes, new_bytes, length, new_len);
}

- (())appendBytes:(ConstPtr<u8>)append_bytes length:(NSUInteger)append_length {
    let old_len = env.objc.borrow::<NSDataHostObject>(this).length;
    let old_bytes = env.objc.borrow::<NSDataHostObject>(this).bytes;
    () = msg![env; this increaseLengthBy:append_length];
    let &NSDataHostObject { bytes, length, .. } = env.objc.borrow(this);
    log_dbg!("appendBytes old_len {}, append_length {}, length {}", old_len, append_length, length);
    log_dbg!("appendBytes old_bytes {:?}, append_bytes {:?}, bytes {:?}", old_bytes, append_bytes, bytes);
    env.mem.memmove(bytes + old_len, append_bytes.cast(), append_length);
}

@end

};

pub fn to_rust_slice(env: &mut Environment, data: id) -> &[u8] {
    let borrowed_data = env.objc.borrow::<NSDataHostObject>(data);
    assert!(!borrowed_data.bytes.is_null() && borrowed_data.length != 0);
    env.mem
        .bytes_at(borrowed_data.bytes.cast(), borrowed_data.length)
    }
