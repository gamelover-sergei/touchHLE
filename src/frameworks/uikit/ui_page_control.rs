use crate::objc::{objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIPageControl: UIControl
- (())setNumberOfPages:(bool)pages {
    log!("TODO: setNumberOfPages:{}", pages);
}

- (())setCurrentPage:(bool)page {
    log!("TODO: setCurrentPage:{}", page);
}

@end

};