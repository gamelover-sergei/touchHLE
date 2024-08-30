use crate::frameworks::foundation::NSUInteger;
use crate::objc::{objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation GKSession: NSObject
- (())initWithSessionID:(NSInteger)session displayName:(bool)_name sessionMode:(bool)mode {
    // TODO
}

@end

};
