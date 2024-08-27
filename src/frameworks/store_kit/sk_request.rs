/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::objc::{ClassExports, HostObject, id, NSZonePtr, nil, release, retain, autorelease};
use crate::{msg_class, objc_classes, msg};
use crate::frameworks::foundation::ns_array::to_vec;
use crate::frameworks::foundation::{NSInteger, NSUInteger};

struct ProductsRequestHostObject {
    product_identifiers: id,
    delegate: id,
}
impl HostObject for ProductsRequestHostObject {}

struct ProductsResponseHostObject {
    products: id,
}
impl HostObject for ProductsResponseHostObject {}

struct ProductHostObject {
    identifier: id,
}
impl HostObject for ProductHostObject {}

struct PaymentHostObject {
    identifier: id,
}
impl HostObject for PaymentHostObject {}

struct PaymentTransactionHostObject {
    payment: id,
}
impl HostObject for PaymentTransactionHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation SKRequest: NSObject
@end

@implementation SKProductsRequest: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(ProductsRequestHostObject {
        product_identifiers: nil,
        delegate: nil
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithProductIdentifiers:(id)prod_ids {
    retain(env, prod_ids);
    env.objc.borrow_mut::<ProductsRequestHostObject>(this).product_identifiers = prod_ids;
    this
}

- (id)initWithProductIdentifiers:(NSUInteger)_identifiers {
    msg![env; this init]
}

- (())setDelegate:(id)delegate {
    env.objc.borrow_mut::<ProductsRequestHostObject>(this).delegate = delegate;
}

- (())start {
    let &ProductsRequestHostObject{delegate, product_identifiers} = env.objc.borrow(this);
    let prod_ids: id = msg![env; product_identifiers allObjects];
    let products = msg_class![env; NSMutableArray array];
    for prod_id in to_vec(env, prod_ids) {
        let product = msg_class![env; SKProduct alloc];
        let product: id = msg![env; product initWithProductIdentifier: prod_id];
        () = msg![env; products addObject:product];
    }
    let resp = msg_class![env; SKProductsResponse alloc];
    let resp: id = msg![env; resp initWithProducts:products];

    () = msg![env; delegate productsRequest:this didReceiveResponse:resp];
    msg![env; delegate requestDidFinish: this]
}

- (())dealloc {
    let &ProductsRequestHostObject{product_identifiers, ..} = env.objc.borrow(this);
    release(env, product_identifiers);

    env.objc.dealloc_object(this, &mut env.mem)
}

@end

@implementation SKProductsResponse: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(ProductsResponseHostObject {
        products: nil
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithProducts:(id)products {
    retain(env, products);
    env.objc.borrow_mut::<ProductsResponseHostObject>(this).products = products;
    this
}

- (id)products {
    env.objc.borrow::<ProductsResponseHostObject>(this).products
}

- (())dealloc {
    let &ProductsResponseHostObject{products} = env.objc.borrow(this);
    release(env, products);

    env.objc.dealloc_object(this, &mut env.mem)
}

@end

@implementation SKProduct: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(ProductHostObject {
        identifier: nil
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithProductIdentifier:(id)prod_id {
    retain(env, prod_id);
    env.objc.borrow_mut::<ProductHostObject>(this).identifier = prod_id;
    this
}

- (id)priceLocale {
    nil
}

- (id)price {
    msg_class![env; NSNumber numberWithInt: 1]
}

- (id)productIdentifier {
    env.objc.borrow::<ProductHostObject>(this).identifier
}

- (id)localizedTitle {
    env.objc.borrow::<ProductHostObject>(this).identifier
}

- (id)localizedDescription {
    env.objc.borrow::<ProductHostObject>(this).identifier
}

- (())dealloc {
    let &ProductHostObject{identifier} = env.objc.borrow(this);
    release(env, identifier);

    env.objc.dealloc_object(this, &mut env.mem)
}
@end

@implementation SKPayment: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(PaymentHostObject {
        identifier: nil
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

+ (id)paymentWithProductIdentifier:(id)prod_id {
    let new = msg![env; this alloc];
    retain(env, prod_id);
    env.objc.borrow_mut::<PaymentHostObject>(new).identifier = prod_id;
    autorelease(env, new)
}

- (id)productIdentifier {
    env.objc.borrow::<PaymentHostObject>(this).identifier
}

- (())dealloc {
    let &PaymentHostObject{identifier} = env.objc.borrow(this);
    release(env, identifier);

    env.objc.dealloc_object(this, &mut env.mem)
}
@end

@implementation SKPaymentTransaction: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(PaymentTransactionHostObject {
        payment: nil
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

+ (id)transactionWithPayment:(id)payment {
    let new = msg![env; this alloc];
    retain(env, payment);
    env.objc.borrow_mut::<PaymentTransactionHostObject>(new).payment = payment;
    autorelease(env, new)
}

- (id)payment {
    env.objc.borrow::<PaymentTransactionHostObject>(this).payment
}

- (())dealloc {
    let &PaymentTransactionHostObject{payment} = env.objc.borrow(this);
    release(env, payment);

    env.objc.dealloc_object(this, &mut env.mem)
}

- (NSInteger)transactionState {
    1
}
@end
};
