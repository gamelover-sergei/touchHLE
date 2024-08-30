use crate::frameworks::foundation::NSInteger;
use crate::objc::{objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation GKSession: NSObject
- (())initWithSessionID:(NSInteger)session displayName:(bool)_name sessionMode:(bool)mode {
    // TODO
}

- (())setAvailable:(bool)available {
    log!("TODO: setAvailable:{}", available);
}

- (())setDelegate:(bool)delegate {
    log!("TODO: setDelegate:{}", delegate);
}

- (())setDataReceiveHandler:(NSInteger)handler withContext:(bool)_context {
    // TODO
}

@end

};
