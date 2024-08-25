use crate::objc::{objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UILoadSaveTableView: UITableView

- (())setSectionHeaderHeight:(bool)height {
    log!("TODO: setSectionHeaderHeight:{}", height);
}

- (())setSectionFooterHeight:(bool)height {
    log!("TODO: setSectionFooterHeight:{}", height);
}

@end

};
