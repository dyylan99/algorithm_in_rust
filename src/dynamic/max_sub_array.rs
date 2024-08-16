
// 53. 最大子数组和  给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    //dp[i] 为包含nums[i]的最大子数组的和
    let mut dp=vec![0 ;nums.len()];
    dp[0]=nums[0];
    let mut res=dp[0];
    for i in 1..nums.len(){
        dp[i]=nums[i].max(dp[i-1]+nums[i]);
        res=res.max(dp[i]);
    }
    res
}