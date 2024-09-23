use crate::objc::{objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UITabBarController: UIControl

- (())setViewControllers:(bool)controllers {
    log!("TODO: setViewControllers:{}", controllers);
}

@end

};
