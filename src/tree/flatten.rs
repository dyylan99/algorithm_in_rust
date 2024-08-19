use std::{cell::RefCell, rc::Rc};

use super::tree_node::TreeNode;


// 定义类型别名
type TreeNodeRef = Rc<RefCell<TreeNode>>;

pub fn flatten(root: &mut Option<TreeNodeRef>) {
    let mut cur = root.clone();
    while let Some(node) = cur {
        let mut node_borrowed = node.borrow_mut();
        if let Some(left) = node_borrowed.left.clone() {
            let mut rightmost = left.clone();
            while let Some(right_node) = rightmost.clone().borrow().right.clone() {
                rightmost = right_node;
            }
            rightmost.borrow_mut().right = node_borrowed.right.clone();
            node_borrowed.right = Some(left.clone());
            node_borrowed.left = None;
        }
        cur = node_borrowed.right.clone();
    }
}