
#[allow(dead_code)]
pub fn valid_palindrome(s: String) -> bool {
    //? fastest solution, although i got the idea for .eq from online so not including.
    // ? also the question was to test two pointer. 
    // let input = s.chars().filter(|x| x.is_alphanumeric())
    // .map(|x| x.to_ascii_lowercase());
    // input.clone().eq(input.rev());

    if s.len() <= 1 {
        return true;
    }
    let input: Vec<char> = s
        .chars()
        .filter(|x| x.is_alphanumeric())
        .map(|x| x.to_ascii_lowercase())
        .collect();
    let j = input.len() - 1;
    for idx in 0..(input.len()) / 2 {
        if input.get(idx) != input.get(j - idx) {
            return false;
        }
    }
    return true;
}
