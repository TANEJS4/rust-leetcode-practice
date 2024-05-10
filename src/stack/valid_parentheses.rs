use std::collections::HashMap;

#[allow(dead_code)]
pub fn valid_parentheses(s: String) -> bool {
    let mut res = false;

    let allowechar = HashMap::from([('}', '{'), (')', '('), (']', '[')]);

    let mut stack: Vec<char> = vec![];
    for chr in s.chars() {
        if vec!['(', '{', '['].contains(&chr) {
            res = false;
            stack.push(chr);
        } else if vec![')', '}', ']'].contains(&chr) && !stack.is_empty() {
            
            if allowechar.get(&chr) == stack.pop().as_ref() {
                res = true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    return res && stack.is_empty();
}
