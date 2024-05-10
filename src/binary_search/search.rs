#[allow(dead_code)]
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = (nums.len()) as i32;
    while left < right {
        let mid: i32 = left + (right - left) / 2;
        let curr: &i32 = nums.get(mid as usize).unwrap();
        if curr == &target {
            println!("Found at {:} ", mid);

            return mid;
        } else if curr > &target {
            right = mid;
            // mid = (right - left) /2;
        } else if curr < &target {
            left = mid + 1;
        } else {
            break;
        }
    }
    println!("Found at {:} ", -1);

    return -1;
}

#[allow(dead_code)]

pub fn search_part2(nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering::{Greater, Less};
    let (mut l, mut r) = (0, nums.len());
    while l < r {
        let i_mid: usize = (l + r) / 2;
        match nums[i_mid].cmp(&target) {
            Less => l = i_mid + 1,
            Greater => r = i_mid,
            _ => return i_mid as i32,
        }
    }
    -1
}
