

//102. 二叉树的层序遍历

use std::{ cell::{RefCell}, rc::Rc};
use std::collections::VecDeque;

use super::tree_node::TreeNode;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res=vec![];
    if root.is_none(){
        return res;
    }
    let mut queue:VecDeque<Rc<RefCell<TreeNode>>>=VecDeque::new();
    queue.push_back(root.unwrap());
    while !queue.is_empty(){
        let mut level=vec![];
        let  size=queue.len();
        for _ in 0..size{
            if let Some(node)=queue.pop_front(){
                let node_borrowed=node.borrow();
                level.push(node_borrowed.val);
                if let Some(left)=&node_borrowed.left{
                    queue.push_back(Rc::clone(left))
                }
                if let Some(right)=&node_borrowed.right{
                    queue.push_back(Rc::clone(right))
                }
            }
        }
        res.push(level);
    }
    res
}
#[test]
fn test_level_order() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.borrow_mut().left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));

    let result = level_order(Some(root));
    assert_eq!(result, vec![vec![1], vec![2, 3], vec![4, 5, 6]]);
}