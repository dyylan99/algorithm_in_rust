

//53. 最大子数组和
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut res=0;
    //dp[i]为以nums[i]结尾的最大子数组和
    let mut dp=vec![0;nums.len()];
    dp[0]=nums[0];
    res=dp[0];
    for i in 1..nums.len(){
        dp[i]=nums[i].max(dp[i-1]+nums[i]);
        res=res.max(dp[i]);
    }
    res

}