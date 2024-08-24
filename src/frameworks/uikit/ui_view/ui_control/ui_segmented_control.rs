/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UISegmentedControl`.

use crate::frameworks::uikit::ui_view::NSUInteger;
use crate::objc::{id, msg, objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UISegmentedControl: UIControl

- (id)initWithItems:(NSUInteger)_items {
    msg![env; this init]
}

- (())setSelectedSegmentIndex:(bool)index {
    log!("TODO: setSelectedSegmentIndex:{}", index);
}

- (())setSegmentedControlStyle:(bool)style {
    log!("TODO: setSegmentedControlStyle:{}", style);
}

- (())setTintColor:(bool)color {
    log!("TODO: setTintColor:{}", color);
}

// TODO: all of it

@end

};
