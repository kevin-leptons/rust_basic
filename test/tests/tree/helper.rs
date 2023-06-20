use rust_basic::Tree;
use testkit::NonZeroSize;

/// Build a tree like this:
///
/// ```txt
/// +--- 0 ---+
/// |    |    |
/// |    |    |
/// v    v    v
/// 1    2    3 ---+
///      |    |    |
///      v    v    v
///      4    5    6 ---+
///                |    |
///                v    v
///                7    8
///                     |
///                     v
///                     9
pub(super) fn non_zero_size_type() -> Tree<NonZeroSize> {
    let mut tree0 = Tree::new(NonZeroSize::new(0));
    let tree1 = Tree::new(NonZeroSize::new(1));
    let mut tree2 = Tree::new(NonZeroSize::new(2));
    let mut tree3 = Tree::new(NonZeroSize::new(3));
    let tree4 = Tree::new(NonZeroSize::new(4));
    let tree5 = Tree::new(NonZeroSize::new(5));
    let mut tree6 = Tree::new(NonZeroSize::new(6));
    let tree7 = Tree::new(NonZeroSize::new(7));
    let mut tree8 = Tree::new(NonZeroSize::new(8));
    let tree9 = Tree::new(NonZeroSize::new(9));
    tree8.add_children([tree9]);
    tree6.add_children([tree7, tree8]);
    tree3.add_children([tree5, tree6]);
    tree2.add_children([tree4]);
    tree0.add_children([tree1, tree2, tree3]);
    return tree0;
}
