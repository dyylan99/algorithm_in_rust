


//416. 分割等和子集
//给你一个 只包含正整数 的 非空 数组 nums 。请你判断是否可以将这个数组分割成两个子集，使得两个子集的元素和相等。
pub fn can_partition(nums: Vec<i32>) -> bool {
   let sum=nums.iter().sum::<i32>() as usize;
    if sum%2!=0 {
        return false;
    }
    let target=sum/2;
    //dp[i]为 当容量为i时最多能装多少的nums, 因此, 当dp[target]=target时,返回true
    let mut dp=vec![0;target+1];
    dp[0]=0;
    for i in 1..nums.len(){
        for j in (nums[i] as usize..=target).rev(){
            dp[j]=dp[j].max(dp[j-nums[i] as usize]+nums[i] as usize);
        }
    }
    return dp[target]==target;

}

#[test]
pub fn test(){
    let nums=vec![1, 2,5];
    let res=can_partition(nums);
}