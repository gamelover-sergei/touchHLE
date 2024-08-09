/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UISlider`.

use crate::objc::{objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UISlider: UIButton

- (())setContinuous:(bool)continuous {
    log!("TODO: setContinuous:{}", continuous);
}

- (())setMinimumValue:(bool)minimum {
    log!("TODO: setMinimumValue:{}", minimum);
}

- (())setMaximumValue:(bool)maximum {
    log!("TODO: setMaximumValue:{}", maximum);
}

- (())setThumbImage:(bool)forState {
    log!("TODO: setThumbImage:{}", forState);
}

// TODO: all of it

@end

};
