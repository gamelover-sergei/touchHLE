use crate::frameworks::foundation::NSUInteger;
use crate::objc::{id, msg, objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation iSimulate: UIView
+ (id)disableTouchesOverlay:(NSUInteger)overlay {
    msg![env; this init]
}

@end

};
