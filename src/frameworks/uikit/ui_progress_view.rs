use crate::objc::{objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIProgressView: UIView

- (())setProgressViewStyle:(bool)style {
    log!("TODO: setProgressViewStyle:{}", style);
}

@end

};
