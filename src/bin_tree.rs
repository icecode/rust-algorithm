use crate::NodeCopy;

pub struct BinTree<T: Ord> {
    root: Option<Box<TreeNode<T>>>,
    len: usize
}

struct TreeNode<T: Ord> {
    data: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>
}

impl <T:Ord> TreeNode<T> {

    fn new(data: T) -> Self {
        TreeNode {
            data,
            left: Option::None,
            right: Option::None
        }
    }
}

impl <T: Ord> BinTree<T> {

    pub fn new() -> Self {
        BinTree {
            root: None,
            len: 0
        }
    }

    pub fn insert(&mut self, node: T) {
        let mut cur:&mut Option<Box<TreeNode<T>>> = &mut self.root;
        let mut ret = false;
        while !ret {
            if cur.is_none() {
                *cur = Option::Some(Box::new(TreeNode::new(node)));
                self.len += 1;
                return
            } else if cur.as_mut().unwrap().as_mut().data > node {
                cur = &mut cur.as_mut().unwrap().left;
            } else {
                cur = &mut cur.as_mut().unwrap().right;
            }
        }
    }

    pub fn find(&self, other:T) -> bool {
        let mut cur = &self.root;
        while let Some(node) = cur {
            if node.data.eq(&other) {
                return true;
            }
            if node.data > other {
                cur = &node.left;
            } else {
                cur = &node.right;
            }
        }
        return false;
    }

    pub fn remove(&mut self, node:T) {

    }

    fn foreach_tree(tree:&TreeNode<T>, apply: fn(e: &T)) {
        if tree.left.is_some() {
            Self::foreach_tree(tree.left.as_ref().unwrap(), apply)
        }
        apply(&tree.data);
        if tree.right.is_some() {
            Self::foreach_tree(tree.right.as_ref().unwrap(), apply)
        }
    }

    pub fn foreach(&self, apply: fn(e: &T)) {
        if self.root.is_some() {
            Self::foreach_tree(self.root.as_deref().unwrap(), apply)
        }
    }
}

#[test]
fn test_bin_tree_find() {
    let mut bin_tree:BinTree<NodeCopy> = BinTree::new();
    bin_tree.insert(NodeCopy{ data: 1});
    bin_tree.insert(NodeCopy{ data: 2});
    bin_tree.insert(NodeCopy{ data: 3});
    bin_tree.insert(NodeCopy{ data: 4});
    bin_tree.insert(NodeCopy{ data: 5});
    bin_tree.insert(NodeCopy{ data: 6});
    bin_tree.insert(NodeCopy{ data: 7});

    assert_eq!(true, bin_tree.find(NodeCopy{data: 1}));
    assert_eq!(true, bin_tree.find(NodeCopy{data: 2}));
    assert_eq!(true, bin_tree.find(NodeCopy{data: 3}));
    assert_eq!(true, bin_tree.find(NodeCopy{data: 4}));
    assert_eq!(true, bin_tree.find(NodeCopy{data: 5}));
    assert_eq!(true, bin_tree.find(NodeCopy{data: 6}));
    assert_eq!(true, bin_tree.find(NodeCopy{data: 7}));
    assert_eq!(false, bin_tree.find(NodeCopy{data: 8}));
    assert_eq!(false, bin_tree.find(NodeCopy{data: 9}));
}