use crate::objc::{id, nil, objc_classes, ClassExports};
use crate::mem::MutPtr;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSOperationQueue: NSObject

- (())addOperation:(id)op { // NSOperation*
    log!("WARNING NSOperationQueue ignoring addOperation: {:?}", op);
}

- (())setSuspended:(bool)suspended {
    log!("TODO: setSuspended:{}", suspended);
}

- (id)operations {
    nil
}

@end

@implementation NSMutableURLRequest: NSObject
+ (id)requestWithURL:(id)url {
    nil
}
@end

@implementation NSURLConnection: NSObject
+ (id)sendSynchronousRequest:(id)request
           returningResponse:(MutPtr<id>)response
                       error:(MutPtr<id>)error {
    nil
}
@end

@implementation NSHTTPURLResponse: NSObject

@end

};
