


//416. 分割等和子集
//给你一个 只包含正整数 的 非空 数组 nums 。请你判断是否可以将这个数组分割成两个子集，使得两个子集的元素和相等。
pub fn can_partition(nums: Vec<i32>) -> bool {
   let sum=nums.iter().sum::<i32>() as usize;
    if sum%2!=0{
        return false;
    }
    let target=sum/2;
    // 背包容量为target+1,则每个物品的价值和重量都为nums[i]的背包问题
    let mut dp=vec![0;target+1];
    dp[0]=0;
    for i in 0..nums.len(){
        for j in (target..=nums[i] as usize).rev(){
            dp[j]=dp[j].max(dp[j-nums[i] as usize]+nums[i] as usize);
        }
    }
    return dp[target]==target
}

#[test]
pub fn test(){
    let nums=vec![1, 5, 11, 5];
    let res=can_partition(nums);
}