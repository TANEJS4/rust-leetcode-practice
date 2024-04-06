use std:: collections::HashMap;

pub fn top_k_freq_elements(nums: Vec<i32>, k: i32)
 -> Vec<i32>
{
    let mut freq = HashMap::<i32, i32>::new();
    for elem in nums {

        if freq.contains_key(&elem) {
            let mut curr_val = freq.get(&elem).unwrap().clone();
            curr_val += 1;
            freq.insert(elem, curr_val);
        } else {
            freq.insert(elem, 1);
        }
    }
    let mut freq_vec = freq.iter().collect::<Vec<(&i32,&i32)>>();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1)) ;
    let res: Vec<i32> =  freq_vec.iter().map(|(k,v)| **k).collect();
    return res[0..usize::try_from(k).unwrap()].to_vec();  
}
