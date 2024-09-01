use crate::objc::{objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UISearchBar: UIView

- (())setShowsCancelButton:(bool)button {
    log!("TODO: setShowsCancelButton:{}", button);
}

@end

};
