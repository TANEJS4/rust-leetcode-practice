use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> 
{
    // let mut result: Vec<i32> = vec![0, 0];
    let mut diff_iter: HashMap<i32, i32>= HashMap::new();
    for (i, elem) in nums.iter().enumerate() {
        diff_iter.insert(*elem, i as i32);
    }

    for idx in 0..nums.len() - 1 {
        // let clo = diff_iter.clone();
        let comp: i32 = target - nums.get(idx).unwrap();
        if diff_iter.contains_key(&comp) && diff_iter.get(&comp).unwrap() != &(idx as i32) {
            return vec![idx.try_into().unwrap(), *diff_iter.clone().get(&comp).unwrap()];
            // }
        }
    //     // print!("Loop {:} -> ", idx + 1);
    //     // let idx_value = nums.get(idx).unwrap();
    //     // print!("idx_value in nums {:?}", idx_value);
    //     // let index = nums
    //     //     .iter()
    //     //     .position(|&item| *diff_iter.get(idx_value).unwrap() == item);
    //     // println!("  index in hash {:?}", index);

    //     // match index {
    //     //     Some(x) => result = vec![idx.try_into().unwrap(), x.try_into().unwrap()],
    //     //     None => (),
    //     // }
    }
    return vec![0, 1];
}
