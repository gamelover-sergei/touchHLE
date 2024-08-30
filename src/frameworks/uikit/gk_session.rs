use crate::frameworks::foundation::NSInteger;
use crate::objc::{objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation GKSession: NSObject
- (())initWithSessionID:(NSInteger)session displayName:(bool)_name sessionMode:(bool)mode {
    // TODO
}

- (())setDelegate:(bool)delegate {
    log!("TODO: setDelegate:{}", delegate);
}

@end

};
