use crate::frameworks::foundation::NSInteger;
use crate::objc::{objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIDatePicker: UIView

- (())setDatePickerMode:(bool)mode {
    log!("TODO: setDatePickerMode:{}", mode);
}

- (())addTarget:(NSInteger)target action:(bool)_action forControlEvents:(bool)_events {
    // TODO
}

@end

};
