/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `MPMediaQuery`.

use crate::frameworks::foundation::NSUInteger;
use crate::objc::{id, msg, nil, objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation MPMediaQuery: NSObject

+ (id)playlistsQuery {
    log!("TODO: [MPMediaQuery playlistsQuery] (not implemented yet)");
    nil
}

+ (id)songsQuery {
    nil
}

@end

@implementation MPMediaPickerController: NSObject
- (id)initWithMediaTypes:(NSUInteger)_types {
    msg![env; this init]
}

@end

};
