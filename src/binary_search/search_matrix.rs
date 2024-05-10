
#[allow(dead_code)]
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) 
-> bool
{
    // base:
    if target < matrix[0][0]{
        // println!("False");
        return false
    }
    //  m is # of rows
    let m = matrix.len();
    let mut m_i =  m -1;
    for i  in 1..m{
        if target < matrix[i][0] && target >= matrix[i-1][0]{
            m_i = i-1;
        } 
    }
    // do the binary search on the index m_i
    use std::cmp::Ordering::{Greater, Less};
    let (mut l, mut r) = (0, matrix[m_i].len());
    while l < r {
        let i_mid: usize = (l + r) / 2;
        match matrix[m_i][i_mid].cmp(&target) {
            Less => l = i_mid + 1,
            Greater => r = i_mid,
            _ => return true,
        }
    }
    false

}
