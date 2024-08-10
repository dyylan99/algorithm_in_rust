
//给定一个包含 n + 1 个整数的数组 nums ，其数字都在 [1, n] 范围内（包括 1 和 n），可知至少存在一个重复的整数。

// 假设 nums 只有 一个重复的整数 ，返回 这个重复的数 。

// 你设计的解决方案必须 不修改 数组 nums 且只用常量级 O(1) 的额外空间。


//解题核心: 如果数组中有重复的元素，那么数组中一定会有环，我们可以使用快慢指针来解决这个问题

pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut slow=nums[0] as usize;
    let mut fast=nums[nums[0] as usize] as usize;
        while slow !=fast {
            slow=nums[slow] as usize;
            fast=nums[nums[fast] as usize] as usize;
        }
        slow=0;
        while slow!=fast {
            slow=nums[slow] as usize;
            fast=nums[fast] as usize;
        }

    slow as i32
}