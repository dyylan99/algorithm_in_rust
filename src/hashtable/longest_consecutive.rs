use std::collections::HashSet;



// 128. 最长连续序列
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut hash_set = HashSet::new();
    let mut res:i32 = 0;
    for ele in nums {
        hash_set.insert(ele);
    }
    for &ele in  &hash_set{
        if !hash_set.contains(&(ele-1)) {
             let mut cur_length=1;
            let mut cur_num=ele;
            while hash_set.contains(&(cur_num+1)) {
                cur_length+=1;
                cur_num+=1;
            }
            res=std::cmp::max(res,cur_length);
        }
    }
    res
}