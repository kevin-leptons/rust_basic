use rust_basic::BinarySearchTree;
use testkit::NonZeroSize;

pub(super) fn sample_small<'a>(
) -> BinarySearchTree<NonZeroSize, NonZeroSize<(&'a str, usize)>> {
    let mut tree = BinarySearchTree::new();
    let border0 = 0;
    let border1 = 251;
    let border2 = 503;
    let border3 = 775;
    let border4 = 1000;
    for i in border0..border1 {
        tree.set(NonZeroSize::new(i), NonZeroSize::new(("value", i)));
    }
    for i in (border1..border2).rev() {
        tree.set(NonZeroSize::new(i), NonZeroSize::new(("value", i)));
    }
    for i in border2..border3 {
        tree.set(NonZeroSize::new(i), NonZeroSize::new(("value", i)));
    }
    for i in (border3..border4).rev() {
        tree.set(NonZeroSize::new(i), NonZeroSize::new(("value", i)));
    }
    return tree;
}
