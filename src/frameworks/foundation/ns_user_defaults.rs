/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSUserDefaults`.
//!
//! References:
//! - Apple's [Preferences and Settings Programming Guide](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/UserDefaults/AboutPreferenceDomains/AboutPreferenceDomains.html).

use super::ns_dictionary::dict_from_keys_and_objects;
use super::ns_string;
use crate::frameworks::foundation::ns_dictionary::DictionaryHostObject;
use crate::objc::{id, msg, msg_class, nil, objc_classes, ClassExports};
use crate::Environment;

#[derive(Default)]
pub struct State {
    /// `NSDictionary*`
    standard_defaults: Option<id>,
}
impl State {
    fn get(env: &mut Environment) -> &mut State {
        &mut env.framework_state.foundation.ns_user_defaults
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSUserDefaults: NSObject

+ (id)standardUserDefaults {
    if let Some(existing) = State::get(env).standard_defaults {
        existing
    } else {
        // TODO: Are there other default keys we need to set?
        let langs_value: id = msg_class![env; NSLocale preferredLanguages];
        let langs_key: id = ns_string::get_static_str(env, "AppleLanguages");
        let new = dict_from_keys_and_objects(env, &[(langs_key, langs_value)]);
        State::get(env).standard_defaults = Some(new);
        new
    }
}

+ (id)resetStandardUserDefaults {
    nil
}

- (id)dictionaryRepresentation {
    this
}

- (bool)boolForKey:(id)defaultName {
    let val: id = msg![env; this objectForKey:defaultName];
    msg![env; val boolValue]
}

- (id)objectForKey:(id)key {
    let host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
    let res = host_obj.lookup(env, key);
    *env.objc.borrow_mut(this) = host_obj;
    res
}

- (())registerDefaults:(id)dict {
    let mut host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(dict));
    for (_, key_value) in host_obj.map {
        let key = key_value[0].0;
        let value = key_value[0].1;
        let mut host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
        host_obj.insert(env, key, value, false);
        *env.objc.borrow_mut(this) = host_obj;
    }
}

// TODO: plist methods etc

@end

};
