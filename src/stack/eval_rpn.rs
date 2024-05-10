#[allow(dead_code)]
pub fn eval_rpn(tokens: Vec<String>) 
-> i32 
{  
    let mut numbers : Vec<i32> = vec![];
    for token in tokens{
        match token.as_str() {
            "+" => {
                let r1 =  numbers.pop().unwrap();
                let r2 = numbers.pop().unwrap();
                numbers.push( r2 + r1);
            },
            "-" => {
                let r1 =  numbers.pop().unwrap();
                let r2 = numbers.pop().unwrap();
                numbers.push(r2 - r1);
            },
            "*" => {
                let r1 =  numbers.pop().unwrap();
                let r2 = numbers.pop().unwrap();
                numbers.push(r2 * r1);
            },
            "/" => {
                let r1 =  numbers.pop().unwrap();
                let r2 = numbers.pop().unwrap();
                numbers.push(r2 / r1);
            },
            _ => {
                numbers.push(token.parse().unwrap());
            }
        }
    }
    return *numbers.last().unwrap();
}
