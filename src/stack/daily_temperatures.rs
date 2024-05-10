#[allow(dead_code)]
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let  mut results: Vec<i32 > = vec![0; temperatures.len()];
        let mut stack = vec![];
        for (i, temp) in temperatures.iter().enumerate(){
            println!("Curr temp {:?}", (i, temp));
            
            while !stack.is_empty() &&  temperatures[*stack.last().unwrap()] < *temp{
                
                println!("Stack, result , vheck {:?}, {:?}  {:?}", stack, ( i - stack.last().unwrap()),  temperatures[*stack.last().unwrap()]);
                let index: usize = stack.pop().unwrap();
                results[index] =( i - index) as i32;
            }
            stack.push(i );

        }
    
    // !  TOO SLOW
    // let stack: Vec<(i32, i32)> = temperatures
    //     .iter()
    //     .enumerate()
    //     .map(|(idx, val)| (*val, idx as i32))
    //     .collect();
    // let mut result: Vec<i32> = vec![];
    // for (idx, temp) in temperatures.iter().enumerate() {
    //     let t = stack.iter().position(|&x|  x.1 > idx as i32 && x.0 > *temp);
    //     if t.is_some() {
    //         result.push(stack[t.unwrap()].1 - idx as i32);
    //     } else {
    //         result.push(0);
    //     }
    // }
    println!("{:?}", results);
    return results;
}
