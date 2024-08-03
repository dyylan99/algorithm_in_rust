use core::hash;
use std::{collections::HashMap, hash::Hash};


// 560. 和为 K 的子数组
//给你一个整数数组 nums 和一个整数 k ，请你统计并返回 该数组中和为 k 的子数组的个数 。
// 子数组是数组中元素的连续非空序列。
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut res=0;
    let mut sum=0;
    //hashmap,key为当前前缀和的值,value为该前缀和出现的次数
    let mut hash_map:HashMap<i32,i32> = HashMap::new();
    //首位数的前缀和为0
    hash_map.insert(0, 1);
    for i in 0..nums.len(){
        sum+=nums[i];
        //如果前缀和sum-k存在,则说明存在一个子数组的和为k
        if let Some(&v)=hash_map.get(&(sum-k)){
            res+=v;
        }
        //更新前缀和sum的次数
        hash_map.insert(sum, hash_map.get(&sum).unwrap_or(&0)+1);
    }
    res
}
#[test]
fn test(){
    let nums = vec![1, 1, 1];
    let k = 2;
    let res = subarray_sum(nums, k);
    assert_eq!(res, 2);
    let nums = vec![1, 2, 3];
    let k = 3;
    let res = subarray_sum(nums, k);
    assert_eq!(res, 2);
    let nums = vec![1];
    let k = 0;
    let res = subarray_sum(nums, k);
    assert_eq!(res, 0);
}