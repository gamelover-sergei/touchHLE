use crate::objc::{objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UISwitch: UIControl

- (())setOn:(bool)on {
    log!("TODO: setOn:{}", on);
}

@end

};
