#[allow(dead_code)]
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut j = numbers.len() - 1;
    let mut i = 0;
    while i <= j {
        println!("current elems  {:} {:} ", numbers[i], numbers[j]);
        if numbers[i] + numbers[j] == target {
            return vec![i as i32 + 1, j as i32 + 1];
        } else if numbers[i] + numbers[j] > target {
            //* slide left */
            println!("slide left j-- ");
            j -= 1;
        } else {
            i += 1;
            //  numbers[i] + numbers[j] < target
            //* slide right */
        }
    }
    // this is never hit as we EXPECT one answer
    return vec![0, 0];
}
