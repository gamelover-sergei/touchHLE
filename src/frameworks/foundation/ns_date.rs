/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSDate`.

use std::time::{Duration, Instant};

use crate::objc::{id, msg_class, objc_classes, ClassExports, HostObject, autorelease};
use super::NSTimeInterval;

struct NSDateHostObject {
    instant: Instant
}
impl HostObject for NSDateHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSDate: NSObject

+ (NSTimeInterval)timeIntervalSinceReferenceDate {
    // This should be consistent with CFAbsoluteTimeGetCurrent()
    // TODO: This should use "Jan 1 2001 00:00:00 GMT" as an absolute reference instead
    let time: NSTimeInterval = msg_class![env; NSProcessInfo systemUptime];
    time
}

+ (id)dateWithTimeIntervalSinceReferenceDate:(NSTimeInterval)ti {
     // TODO: This should use "Jan 1 2001 00:00:00 GMT" as an absolute reference instead
    let host_object = Box::new(NSDateHostObject {
        instant: env.startup_time + Duration::from_secs(ti as u64)
    });
    let new = env.objc.alloc_object(this, host_object, &mut env.mem);
    autorelease(env, new)
}

@end

};