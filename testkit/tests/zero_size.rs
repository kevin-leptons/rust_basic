use testkit::ZeroSize;

#[test]
fn new() {
    let _ = ZeroSize::new();
}

#[test]
fn equal() {
    let instance1 = ZeroSize::new();
    let instance2 = ZeroSize::new();
    assert_eq!(instance1, instance2);
}

#[test]
fn clone() {
    let instance1 = ZeroSize::new();
    let instance2 = instance1.clone();
    assert_eq!(instance1, instance2);
}

#[test]
fn default() {
    let instance1 = ZeroSize::default();
    let instance2 = ZeroSize::new();
    assert_eq!(instance1, instance2);
}
