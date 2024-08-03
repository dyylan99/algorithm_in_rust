


//416. 分割等和子集
//给你一个 只包含正整数 的 非空 数组 nums 。请你判断是否可以将这个数组分割成两个子集，使得两个子集的元素和相等。
pub fn can_partition(nums: Vec<i32>) -> bool {
    let sum = nums.iter().sum::<i32>() as usize;
    if sum%2!=0{
        return false;
    }
    //判断是否可以选出一些数,使它们和为sum/2
    //问题可以转化为0,1背包问题,每个数只能取一次,背包容量为sum/2
    //声明一个长度为sum/2的数组,数组的每个元素表示是否可以选出一些数,使它们和为sum/2
    // dp[i]表示是否可以选出一些数,使它们和为i
    let mut dp = vec![false; sum/2+1];     
    dp[0] = true;
    for num in nums{
        for i in (num as usize..=sum/2).rev(){
            //dp[i-num]表示是否可以选出一些数,使它们和为i-num,因为此时遍历到num,所以可以选出一些数,使它们和为i
            dp[i] = dp[i] || dp[i-num as usize];
        }
    }
    dp[sum/2]
}