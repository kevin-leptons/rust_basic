use rust_basic::RedBlackTree;
use testkit::NonZeroSize;

pub(super) fn sample_large<'a>(
) -> RedBlackTree<NonZeroSize, NonZeroSize<(&'a str, usize)>> {
    let mut tree = RedBlackTree::new();
    let border0 = 0;
    let border1 = 2501;
    let border2 = 5003;
    let border3 = 7537;
    let border4 = 10000;
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
