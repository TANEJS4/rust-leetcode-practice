#[allow(dead_code)]
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {

    let max = piles.iter().cloned().fold(1, i32::max) + 1;
    let k_min = (piles.iter().map(|&e| e as i64).sum::<i64>()) / h as i64;
    use std::cmp::Ordering::{Greater, Less};
    let (mut l, mut r) = (k_min, max as i64);
    while l < r {
        let i_mid = (l + r) / 2;
        let time_taken: i32 = piles
            .iter()
            .cloned()
            .map(|x| (x as f64 / i_mid as f64).ceil() as i32)
            .sum();

        match time_taken.cmp(&h) {
            Greater => {
                l = i_mid + 1;
            }
            Less => {
                r = i_mid;
            }
            _ => {
                r = i_mid;
            }
        }
    }
    println!("l - {:}", l);
    return l as i32;
}
