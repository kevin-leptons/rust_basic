use testkit::NonZeroSize;

#[test]
fn new() {
    let value = 137;
    let instance = NonZeroSize::new(value);
    assert_eq!(instance.value, value);
}

#[test]
fn equal() {
    let value = 137;
    let instance1 = NonZeroSize::new(value);
    let instance2 = NonZeroSize::new(value);
    assert_eq!(instance1, instance2);
}

#[test]
fn not_equal() {
    let instance1 = NonZeroSize::new(137);
    let instance2 = NonZeroSize::new(299);
    assert_ne!(instance1, instance2);
}

#[test]
fn clone() {
    let instance1 = NonZeroSize::new(137);
    let instance2 = instance1.clone();
    assert_eq!(instance1, instance2);
}

// This test does nothing but creating an instance to trigger memory release
// process. The test can not work alone, it requries an external tool such as
// Valgrind to diagnose memory issues.
//
// Warn: The test maybe still passed even memory release process has issues.
#[test]
fn drop() {
    let _ = NonZeroSize::new(137);
}
