use std::collections::HashSet;



// 128. 最长连续序列
//给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。
// 请你设计并实现时间复杂度为 O(n) 的算法解决此问题。
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut hash_set = HashSet::new();
    for num in nums.iter(){
        hash_set.insert(*num);
    }
    let mut res = 0;
    nums.iter().for_each(|&num|{
        if !hash_set.contains(&(num-1)){ //这一步主要是为了从最小的数字开始遍历, 防止一些重复步骤
            let mut cur_num=num;
            let mut cur_len=1;
            while hash_set.contains(&(cur_num+1)) {
                cur_len+=1;
                cur_num+=1;
            }
            res=res.max(cur_len);
        }
    });
    res
}