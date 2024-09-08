use std::collections::HashSet;



// 128. 最长连续序列
//给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。
// 请你设计并实现时间复杂度为 O(n) 的算法解决此问题。
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut res=0;
    let mut set=HashSet::new();
    for num in nums.iter(){
        set.insert(*num);
    }
    for num in nums.iter(){
        if !set.contains(&(num-1)) {
            let mut cur_nums = *num;
            let mut cur_len = 1;
            while set.contains(&(&cur_nums+1)) {
                cur_nums+=1;
                cur_len+=1;
            }
            res=res.max(cur_len);
        }
    }
    res
}

#[test]
pub fn test(){
    let longest_consecutive = longest_consecutive(vec![5,3,4,8,11,2,12]);
    println!("{}",longest_consecutive)
}