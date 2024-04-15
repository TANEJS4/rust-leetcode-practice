#[allow(dead_code)]
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    //* count zeros
    let mut zeros: i32 = 0;
    let mut product = 1;

    let mut prod_nums = vec![0; nums.len()];
    for i in &nums {
        if i == &0 {
            zeros += 1;
        } else {
            product *= i;
        }
    }
    if zeros == 1 {
        for idx in 0..nums.len() {
            prod_nums[idx] = if nums[idx] == 0 { product } else { 0 }
        }
    } else if zeros == 0 {
        for idx in 0..nums.len() {
            prod_nums[idx] = product / nums[idx];
        }
    }
    // for idx in 0..nums.len() {
    //     if nums.get(idx).unwrap() == &0{
    //         prod_nums.push(product);
    //         continue;
    //     }

    //     let mut clone_nums: Vec<i32> = nums.clone();
    //     clone_nums.remove(idx);
    //     let temp_prod = clone_nums.iter().product();
    //     //.copied().reduce(|a, b| a * b).unwrap();
    //     prod_nums.insert(idx, temp_prod);
    // }
    return prod_nums;
}
