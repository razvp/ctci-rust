
#[test]
fn test_minimal_tree() {
    use crate::data_structures::tree::BSTree;
    let v = [1, 2, 3, 5, 6, 7];
    let bstree = BSTree::minimal_bstree_from_slice(&v);

    dbg!(&bstree);
    // bstree.print();
}
