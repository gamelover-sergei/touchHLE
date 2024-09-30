/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSCalendar`.

use crate::frameworks::foundation::NSUInteger;
use crate::objc::{id, msg, nil, objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSCalendar: NSObject

+ (id)autoupdatingCurrentCalendar:(NSUInteger)calendar {
    msg![env; this init]
}

+ (id)currentCalendar { // NSCalendar*
    nil
}

@end

};
