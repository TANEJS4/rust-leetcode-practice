
#[allow(dead_code)]
pub fn find_min(nums: Vec<i32>) 
-> i32 
{
    return nums.iter().cloned().fold(i32::MAX, i32::min) ;

}
