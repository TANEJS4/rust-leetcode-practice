#[allow(dead_code)]
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let input = nums.clone();
    
    // let mut i =  0;
    for i in 0..nums.len() - 1 {
        let mut sorted = input[i + 1..].to_vec();
        let target = -nums[i];

        let mut temp: Vec<Vec<i32>> = {
            let mut result: Vec<Vec<i32>> = vec![];
            sorted.sort();
            let mut j = sorted.len() - 1;
            let mut i = 0;
            while i < j {
                if sorted[i] + sorted[j] == target {
                    result.push(vec![sorted[i], sorted[j]]);
                    println!("RESULT {:?} {:?} ", result, -target);
                    i += 1;
                } else if sorted[i] + sorted[j] > target {
                    //* slide left */
                    j -= 1;
                } else {
                    //* slide right */
                    i += 1;
                }
            }
            result
        };
        if temp.len()>0 {
            let _ = temp
                .iter_mut()
                .map(|elem| {
                    elem.insert(0, nums[i]);
                    elem.sort();
                })
                .collect::<Vec<_>>();
            temp.dedup();
            println!("Some= {:?}", temp);

            result.append(&mut temp.clone());
            println!("res  found {:?},  {:?}, ", i, result);
        }
    }

    result.sort();
    result.dedup();
    return result;
}
