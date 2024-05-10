#[allow(dead_code)]
pub fn generate_parenthesis(n: i32)
// -> Vec<String>
{
    println!("->> {:?}", generate("".to_owned(), n, 0, 0));
    return;
}
fn generate(res: String, n: i32, opener: i32, closer: i32) -> Vec<String> {
    // base cases;
    if closer > opener || opener > n || closer > n {
        return vec![];
    }
    if opener == n && closer == n {
        return vec![res];
    }
    // add opening bracket
    let mut build_left = res.to_owned();
    build_left.push('(');
    let mut left = generate(build_left, n, opener + 1, closer);

    // add closing bracket
    let mut build_right = res.to_owned();
    build_right.push(')');
    let right = generate(build_right, n, opener, closer + 1);

    left.extend(right);
    return left;
}
