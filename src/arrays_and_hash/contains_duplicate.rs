use std::collections::HashMap;
#[allow(dead_code)]
pub fn contains_duplicate(nums: Vec<i32>) -> bool 
{
    let mut counter = HashMap::<i32,i32>::new();
    for i in nums {
        if counter.contains_key(&i){
            let ref mut x = counter.get_mut(&i).unwrap();
            **x +=1;
            return true
        } 
        else {
        counter.insert(i,1);
        };

    }    
    return false;
}
