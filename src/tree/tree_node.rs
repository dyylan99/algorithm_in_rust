
use crate::TreeNodeRef;

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct TreeNode{
    pub val:i32,
    pub left:Option<TreeNodeRef>,
    pub right:Option<TreeNodeRef>,
}
impl TreeNode {
    #[inline]
    pub fn new(val:i32)->Self{
        TreeNode{
            val,
            left:None,
            right:None,
        }
    }
}
