/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIWebView`.

use crate::frameworks::core_graphics::CGRect;
use crate::impl_HostObject_with_superclass;
use crate::objc::{
    id, msg, msg_super, nil, objc_classes, ClassExports, NSZonePtr, SEL,
};

struct UIWebViewHostObject {
    superclass: super::UIControlHostObject,
    delegate: id,
}
impl_HostObject_with_superclass!(UIWebViewHostObject);
impl Default for UIWebViewHostObject {
    fn default() -> Self {
        UIWebViewHostObject {
            superclass: Default::default(),
            delegate: nil,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIWebView: UIControl

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIWebViewHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithFrame:(CGRect)frame {
    let this: id = msg_super![env; this initWithFrame:frame];
    this
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder: coder];
    this
}

- (())dealloc {
    let UIWebViewHostObject { .. } = std::mem::take(env.objc.borrow_mut(this));

    msg_super![env; this dealloc]
}

- (())setDelegate:(id)delegate { // something implementing UIWebViewDelegate
    log_dbg!("setDelegate:{:?}", delegate);
    let host_object = env.objc.borrow_mut::<UIWebViewHostObject>(this);
    host_object.delegate = delegate;
}

- (id)delegate {
    env.objc.borrow::<UIWebViewHostObject>(this).delegate
}

- (())setScalesPageToFit:(bool)fit {
    log!("TODO: setScalesPageToFit:{}", fit);
}

- (())setDetectsPhoneNumbers:(bool)numbers {
    log!("TODO: setDetectsPhoneNumbers:{}", numbers);
}

- (())stopLoading {
    log_dbg!("UIWebView stopLoading");

    let delegate: id = env.objc.borrow::<UIWebViewHostObject>(this).delegate;
    if delegate != nil {
        let sel: SEL = env.objc.register_host_selector("webViewDidFinishLoad:".to_string(), &mut env.mem);
        let responds: bool = msg![env; delegate respondsToSelector:sel];
        if delegate != nil && responds {
            let _ : () = msg![env; delegate webViewDidFinishLoad:this];
        }
    }
}

- (bool)becomeFirstResponder {
    true
}

- (bool)resignFirstResponder {
    true
}

@end

};
