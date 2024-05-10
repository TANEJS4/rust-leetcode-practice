use std::cmp::{max, min};
#[allow(dead_code)]
pub fn max_area(height: Vec<i32>)
 -> i32 
{
    println!("{:}", height.len());
    let mut curr_area = 0;
    let  mut left = 0;
    let mut right = height.len() -1 ;
    while left < right{
        // calculate the area
        let area = min(height[left], height[right]) * (right - left) as i32;
        println!("{:?}", Vec::from([height[left], height[right],  (right - left) as i32, area, curr_area]));
        curr_area =  max(area, curr_area);
        if height[left] < height[right]{
            left+=1;
        } else {
            right -=1;
        }
    }
    return curr_area;
}

// 1 * 