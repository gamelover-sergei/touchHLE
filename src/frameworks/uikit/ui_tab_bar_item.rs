use crate::frameworks::foundation::NSInteger;
use crate::objc::{objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UITabBarItem: UIControl

- (())initWithTitle:(NSInteger)title image:(bool)_image tag:(bool)_tag {
  // TODO
}

- (())initWithTabBarSystemItem:(NSInteger)item tag:(bool)_tag {
  // TODO
}

@end

};
