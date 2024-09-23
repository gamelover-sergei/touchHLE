use crate::objc::{id, nil, objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UITabBarController: UIControl

- (id)view {
    nil
}

- (())setViewControllers:(bool)controllers {
    log!("TODO: setViewControllers:{}", controllers);
}

@end

};
