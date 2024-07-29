use std::collections::HashMap;



pub fn two_sum(nums:Vec<i32>,target:i32)->Vec<i32>{
    let mut hash_map = HashMap::new();
    for(index,&num) in nums.iter().enumerate(){
        let a=target-num;
        if hash_map.contains_key(&a){
            return vec![hash_map[&a],index as i32];
        }
        else{
            hash_map.insert(num,index as i32);
        }
    }
    vec![]
}