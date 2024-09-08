use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use crate::TreeNodeRef;

use super::tree_node::TreeNode;



pub struct Solution{

}
// impl Solution {

//     pub fn is_valid_bst(root: Option<TreeNodeRef>) -> bool {
//         fn validate(node: Option<TreeNodeRef>, pre: &mut Option<TreeNodeRef>) -> bool {
//             if let Some(current) = node {
//                 let current_borrowed = current.borrow();

//                 // Check left subtree
//                 if !validate(current_borrowed.left.clone(), pre) {
//                     return false;
//                 }

//                 // Check current node
//                 if let Some(pre_node) = pre {
//                     if pre_node.borrow().val >= current_borrowed.val {
//                         return false;
//                     }
//                 }

//                 // Update pre to current node
//                 *pre = Some(current.clone());

//                 // Check right subtree
//                 if !validate(current_borrowed.right.clone(), pre) {
//                     return false;
//                 }
//             }
//             true
//         }

//         let mut pre: Option<Rc<RefCell<TreeNode>>> = None;
//         validate(root, &mut pre)
//     }
// }      

