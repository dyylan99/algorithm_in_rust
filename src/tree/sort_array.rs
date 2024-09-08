use core::num;



pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    let mut nums=nums.clone();
    let  n=nums.len();
    for i in (0..n/2).rev() {
        sift_down(&mut nums, n, i);
    }
    heapSort(&mut nums);
    nums
}


pub fn sift_down(nums:&mut Vec<i32>,heapSize:usize,mut k:usize){
    //非叶子节点,需要一直下沉
    while k<heapSize/2 {
        let mut largest:usize=2*k+1;
        let right=2*k+2;
        if right<heapSize && nums[right]>nums[largest] {
            largest=right;
        }
        if nums[k]>nums[largest] {
            break;
        }
        nums.swap(k,largest);
        k=largest;
    }
}
pub fn heapSort( nums:&mut Vec<i32>){
    let n=nums.len();
    for k in (0..n).rev()   {
        nums.swap(0,k);
        sift_down(nums, k, 0)
    }
}
#[test]
pub fn test(){
    let sort_array = sort_array(vec![5,3,4,8,11,2,12]);
    println!("{:?}",sort_array)
}