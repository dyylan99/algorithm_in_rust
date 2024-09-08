
//283. 移动零
//给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
// 请注意 ，必须在不复制数组的情况下原地对数组进行操作。
pub fn move_zeroes(nums: &mut Vec<i32>) {
    //j:记录0的位置,遇到非0的数就和i位置的0交换, i:始终向前移动
    let mut j=0;
    for i in 0..nums.len(){
        if nums[i]!=0{
            nums.swap(i, j);
            j+=1;
        }
    }
}