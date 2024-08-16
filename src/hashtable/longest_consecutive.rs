use core::num;
use std::collections::HashSet;



// 128. 最长连续序列
//给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。
// 请你设计并实现时间复杂度为 O(n) 的算法解决此问题。
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut res=0;
    let mut set=HashSet::new();
    for ele in nums.iter() {
         set.insert(*ele);
    }
    nums.iter().for_each(|&num|{
        if !set.contains(&(num-1)){  //为了找到连续数字的最小的那一个, 避免重复
            let mut cur_len=1;
            let mut cur_num=num;
            while set.contains(&(cur_num+1)) {
                cur_len+=1;
                cur_num+=1;
            }
            res=res.max(cur_len);
        }
    });
    res
}

#[test]
pub fn test(){
    let longest_consecutive = longest_consecutive(vec![5,3,4,8,11,2,12]);
    println!("{}",longest_consecutive)
}