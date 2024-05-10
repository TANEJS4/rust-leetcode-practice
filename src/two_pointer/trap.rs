use std::cmp::min;
#[allow(dead_code)]

pub fn trap(height: Vec<i32>) 
-> i32
{
    //  alg would be 
    // min(max(height towards Left of index i ), max(height towards right of index i))
    //                               - height of i
    let mut max_trap = 0;
    for i  in 1..height.len() -1 {
        let  left_min = *height[0..i].iter().max().unwrap_or(&0);
        let  right_min = *height[i+1..].iter().max().unwrap_or(&0);

        let curr_capacity =  min(left_min, right_min) - height[i];
        println!("> {:?}", Vec::from([left_min, right_min, height[i], curr_capacity, max_trap]));
        
        if curr_capacity <=0 {
            continue;
        } else {
            max_trap += curr_capacity;
        }
    }
    println!("result {:}", max_trap);
    return max_trap;
}

