/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSDate`.

use super::{ns_string, NSComparisonResult, NSTimeInterval};
use crate::frameworks::core_foundation::time::apple_epoch;
use crate::frameworks::foundation::{NSInteger, NSUInteger};
use crate::objc::{autorelease, id, msg, msg_class, objc_classes, retain, ClassExports, HostObject, NSZonePtr};

use std::ops::Add;
use std::time::{Duration, SystemTime};

struct NSDateHostObject {
    time_interval: NSTimeInterval,
}
impl HostObject for NSDateHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSDate: NSObject

+ (NSTimeInterval)timeIntervalSinceReferenceDate {
    SystemTime::now()
        .duration_since(apple_epoch())
        .unwrap()
        .as_secs_f64()
}

+ (id)date {
    // "Date objects are immutable, representing an invariant time interval
    // relative to an absolute reference date (00:00:00 UTC on 1 January 2001)."
    let time_interval = SystemTime::now()
        .duration_since(apple_epoch())
        .unwrap()
        .as_secs_f64();
    let host_object = Box::new(NSDateHostObject {
        time_interval
    });
    let new = env.objc.alloc_object(this, host_object, &mut env.mem);
    log_dbg!("[NSDate date] => {:?} ({:?}s)", new, time_interval);
    autorelease(env, new)
}

+ (id)dateWithTimeIntervalSinceNow:(NSUInteger)_now {
    msg![env; this init]
}

+ (id)dateWithTimeIntervalSince1970:(NSUInteger)_since1970 {
    msg![env; this init]
}

+ (id)distantFuture {
    let time_interval = SystemTime::now()
        .duration_since(apple_epoch())
        .unwrap()
        .as_secs_f64() * 2.0;
    let host_object = Box::new(NSDateHostObject {
        time_interval
    });
    let new = env.objc.alloc_object(this, host_object, &mut env.mem);

    log_dbg!("[(NSDate*){:?} distantFuture]: date {:?}", this, new);

    autorelease(env, new)
}

+ (id)distantPast {
    let time_interval = SystemTime::now()
        .duration_since(apple_epoch())
        .unwrap()
        .as_secs_f64() * 2.0;
    let host_object = Box::new(NSDateHostObject {
        time_interval
    });
    let new = env.objc.alloc_object(this, host_object, &mut env.mem);

    log_dbg!("[(NSDate*){:?} distantPast]: date {:?}", this, new);

    autorelease(env, new)
}

+ (NSTimeInterval)timeIntervalSinceReferenceDate {
    let now: id = msg_class![env; NSDate date];
    msg![env; now timeIntervalSinceReferenceDate]
}

- (NSTimeInterval)timeIntervalSinceDate:(id)anotherDate {
    // assert!(!anotherDate.is_null());
    let host_object = env.objc.borrow::<NSDateHostObject>(this);
    let another_date_host_object = env.objc.borrow::<NSDateHostObject>(anotherDate);
    let result =  host_object.time_interval-another_date_host_object.time_interval;
    log_dbg!("[(NSDate*){:?} ({:?}s) timeIntervalSinceDate:{:?} ({:?}s)] => {}", this, host_object.time_interval, anotherDate, another_date_host_object.time_interval, result);
    result
}

// NSCopying implementation
- (id)copyWithZone:(NSZonePtr)_zone {
    retain(env, this)
}

- (id)addTimeInterval:(NSTimeInterval)seconds {
    let curr = env.objc.borrow::<NSDateHostObject>(this).time_interval;
    let host_object = Box::new(NSDateHostObject {
        time_interval: curr + seconds,
    });
    let isa = env
        .objc
        .get_known_class("NSDate", &mut env.mem);
    let new = env.objc.alloc_object(isa, host_object, &mut env.mem);
    autorelease(env, new)
}

- (id)initWithTimeIntervalSinceNow:(NSUInteger)_now {
    msg![env; this init]
}

- (id)initWithTimeIntervalSinceReferenceDate:(NSUInteger)_date {
    msg![env; this init]
}

- (NSComparisonResult)compare:(id)other { // NSDate *
    let a = env.objc.borrow::<NSDateHostObject>(this).time_interval;
    let b = env.objc.borrow::<NSDateHostObject>(other).time_interval;
    return ns_string::from_rust_ordering(a.total_cmp(&b));
}

- (NSTimeInterval)timeIntervalSinceReferenceDate {
    env.objc.borrow::<NSDateHostObject>(this).time_interval
}

- (NSTimeInterval)timeIntervalSinceNow {
    let host_object = env.objc.borrow::<NSDateHostObject>(this);
    let time_interval = SystemTime::now()
        .duration_since(apple_epoch())
        .unwrap()
        .as_secs_f64();
    time_interval - host_object.time_interval
}

- (NSTimeInterval)timeIntervalSince1970 {
    let time_interval = env.objc.borrow::<NSDateHostObject>(this).time_interval;
    apple_epoch()
        .add(Duration::from_secs_f64(time_interval))
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

- (id)addTimeInterval:(NSTimeInterval)seconds {
    let interval = env.objc.borrow::<NSDateHostObject>(this).time_interval + seconds;
    let date = msg_class![env; NSDate date];
    env.objc.borrow_mut::<NSDateHostObject>(date).time_interval = interval;
    date
}

- (id)isEqualToDate:(NSUInteger)_date {
    msg![env; this init]
}

- (())descriptionWithCalendarFormat:(NSInteger)format timeZone:(bool)_zone locale:(bool)_locale {
    // TODO
}

- (())UTF8String {

}

@end

@implementation NSDateComponents: NSDate
- (())setHour:(bool)hour {
    log!("TODO: setHour:{}", hour);
}

- (())setMinute:(bool)minute {
    log!("TODO: setMinute:{}", minute);
}

- (())setSecond:(bool)second {
    log!("TODO: setSecond:{}", second);
}

@end

@implementation ReachabilityQuery: NSDate

// NSCopying implementation
- (id)copyWithZone:(NSZonePtr)_zone {
    retain(env, this)
}

@end

};
