use crate::frameworks::foundation::NSInteger;
use crate::objc::{objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation TransitionView: UIView

- (())insertSubview:(NSInteger)sub atIndex:(bool)_index {
    // TODO
}

@end

};
