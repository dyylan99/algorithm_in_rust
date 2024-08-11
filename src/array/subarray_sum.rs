use core::hash;
use std::{collections::HashMap, hash::Hash};


// 560. 和为 K 的子数组
//给你一个整数数组 nums 和一个整数 k ，请你统计并返回 该数组中和为 k 的子数组的个数 。
// 子数组是数组中元素的连续非空序列。
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    //map的key是前缀和的值, value是前缀和为该值的子数组个数
   let mut map=HashMap::new();
   let mut res=0;
    let mut sum=0;
    map.insert(0, 1);
    for i in 0..nums.len(){
        sum+=nums[i];
        if map.contains_key(&(sum-k)) {
            res+=map.get(&(sum-k)).unwrap();
        }
        *map.entry(sum).or_insert(0)+=1;
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