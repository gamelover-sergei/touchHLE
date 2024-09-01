use crate::objc::{objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UISearchBar: UIView

- (())setShowsCancelButton:(bool)button {
    log!("TODO: setShowsCancelButton:{}", button);
}

- (())setAutocorrectionType:(bool)_type {
    log!("TODO: setAutocorrectionType:{}", _type);
}

- (())setAutocapitalizationType:(bool)_type {
    log!("TODO: setAutocapitalizationType:{}", _type);
}

@end

};
