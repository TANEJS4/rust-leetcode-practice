use std::collections::HashMap;
#[allow(dead_code)]
pub fn group_anagrams(strs: Vec<String>) 
// ->HashMap<String, Vec<String>> 
-> Vec<Vec<String>>
{
    let mut hash: HashMap<String, Vec<String>> = HashMap::<String, Vec<String>>::new();
    for str in strs{
        let mut sort_str:Vec<char> = str.chars().collect();
        sort_str.sort();
        let sorted_str: String = String::from_iter(sort_str);
        println!("{:}",sorted_str);
        if hash.contains_key(&sorted_str){
             let mut curr_val: Vec<String> = hash.get(&sorted_str).unwrap().clone();
            curr_val.push( str);    
            hash.insert(sorted_str, curr_val);
        } else {
            hash.insert(sorted_str, vec![str]);
        }
    }
    return hash.into_values().collect();
}