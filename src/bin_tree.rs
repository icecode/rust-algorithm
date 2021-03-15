

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