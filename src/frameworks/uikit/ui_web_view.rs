use crate::objc::{id, nil, objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIWebView: UIView

- (id)loadRequest {
    nil
}

- (())setScalesPageToFit:(bool)fit {
    log!("TODO: setScalesPageToFit:{}", fit);
}

- (())setDelegate:(bool)delegate {
    log!("TODO: setDelegate:{}", delegate);
}

- (())setDetectsPhoneNumbers:(bool)numbers {
    log!("TODO: setDetectsPhoneNumbers:{}", numbers);
}

@end

};
