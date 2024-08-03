use std::{collections::HashMap, vec};

pub fn two_sum(nums:Vec<i32>,target:i32)->Vec<i32>{
    let mut hash_map=HashMap::new();
    for i in 0..nums.len(){
        if let Some(&v)=hash_map.get(&(target-nums[i])){
            return vec![v as i32,i as i32];
        }
        hash_map.insert(nums[i],i);
    }
    vec![]
}