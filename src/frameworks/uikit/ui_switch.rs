use crate::objc::{id, nil, objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UISwitch: UIControl

- (id)isOn {
    nil
}

- (())setOn:(bool)on {
    log!("TODO: setOn:{}", on);
}

@end

};
