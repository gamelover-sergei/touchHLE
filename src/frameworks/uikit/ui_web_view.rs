use crate::frameworks::foundation::{NSUInteger, NSInteger};
use crate::objc::{id, msg, nil, objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIWebView: UIView

+ (id)instanceMethodSignatureForSelector:(NSUInteger)selector {
    msg![env; this init]
}

+ (id)methodReturnType {
    nil
}

+ (id)numberOfArguments {
    nil
}

- (id)loadRequest:(NSUInteger)request {
    msg![env; this init]
}

- (id)request {
    nil
}

- (id)stringByEvaluatingJavaScriptFromString:(NSUInteger)string {
    msg![env; this init]
}

- (())loadHTMLString:(NSInteger)string baseURL:(bool)_url {
    // TODO
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
