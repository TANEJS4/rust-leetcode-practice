pub fn longest_consecutive(nums: Vec<i32>)
-> i32
{
    println!("" );
    println!("--------------------" );
    println!("running for sequence: {:?}", nums );
    
    let mut sorted_num = nums.clone();
    sorted_num.sort();
    sorted_num.dedup();
    println!("   sorted sequence: {:?}", sorted_num );
    if sorted_num.len() <= 1 {
        return sorted_num.len() as i32;
    }
    let mut counter: i32 = 1;
    let mut max: i32 = 0;
    for idx in 0..sorted_num.len() - 1 {
        if sorted_num[idx] + 1 == sorted_num[idx + 1] {
            counter += 1;
        } else {
            counter = 1;
        }
        if counter >= max {
            max = counter;
        }
        println!("idx-> {:}, elem -> {:},  counter -> {:}, max -> {:}", idx, sorted_num[idx], counter, max);
    }

    return max;
}
