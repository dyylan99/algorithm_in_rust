use std::collections::HashMap;


// 560. 和为 K 的子数组
//给你一个整数数组 nums 和一个整数 k ，请你统计并返回 该数组中和为 k 的子数组的个数 。
// 子数组是数组中元素的连续非空序列。
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut map = HashMap::new();
    //首位数前缀和为0
    map.insert(0, 1);
    let mut sum = 0;
    let mut res = 0;
    for ele in nums {
        sum += ele;
        //sum - b = k => sum = b + k, 当前的前缀和为sum的个数是定值, 只要找到前缀和为sum-k的个数, 即可得到中间那段前缀和为k的子数组个数
        //因此 fn(sum=k)=fn(sum)-fn(sum-k),因此,需要找到map中存储的key为sum-k
        if let Some(&v) = map.get(&(sum - k)) {
            res += v;
        }
        *map.entry(sum).or_insert(0) += 1;
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