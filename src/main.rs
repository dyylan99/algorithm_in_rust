use std::{cell::RefCell, rc::Rc};

use tree::tree_node::TreeNode;

use crate::bit::is_unique::is_unique;

pub mod bit;
mod graph;
mod hashtable;
mod string;
mod array;
pub mod dynamic;
pub mod tree;

type TreeNodeRef = Rc<RefCell<TreeNode>>;

fn main() {
    let is_unique = is_unique("abcc".to_string());
    println!("{}", is_unique);
    println!("Hello, world!");
}
