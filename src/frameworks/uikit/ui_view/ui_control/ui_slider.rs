/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UISlider`.

use crate::frameworks::uikit::ui_view::NSInteger;
use crate::objc::{id, nil, objc_classes, ClassExports};

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

- (())setValue:(bool)value {
    log!("TODO: setValue:{}", value);
}

- (())setMinimumValueImage:(bool)image {
    log!("TODO: setMinimumValueImage:{}", image);
}

- (())setMaximumValueImage:(bool)image {
    log!("TODO: setMaximumValueImage:{}", image);
}

- (())setThumbImage:(NSInteger)image forState:(bool)_state {
    // TODO
}

- (())setMinimumTrackImage:(NSInteger)image forState:(bool)_state {
    // TODO
}

- (())setMaximumTrackImage:(NSInteger)image forState:(bool)_state {
    // TODO
}

- (())setValue:(NSInteger)value animated:(bool)_animated {
    // TODO
}

- (id)value {
    nil
}

- (id)isDragging {
    nil
}
// TODO: all of it

@end

@implementation Slider: UISlider
- (id)isDragging {
    nil
}
@end

};
