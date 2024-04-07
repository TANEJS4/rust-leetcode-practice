use std::collections::HashSet;

pub fn valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut row_hash = HashSet::<(u32, u32)>::with_capacity(9);
    let mut col_hash = HashSet::<(u32, u32)>::with_capacity(9);
    let mut box_hash = vec![HashSet::<u32>::with_capacity(9); 9];
    for row_idx in 0..9 {
        for col_idx in 0..9 {
            if let Some(digit) = board[row_idx][col_idx].to_digit(10) {
                // * check for duplicate in the rows
                if check_in_hash(&row_hash, (row_idx.try_into().unwrap(), digit)) {
                    
                    println!("row hash is false" );
                    
                    return false;
                } else {
                    row_hash.insert((row_idx.try_into().unwrap(), digit));
                }

                // * check for duplicate in the columns
                if check_in_hash(&col_hash, (col_idx.try_into().unwrap(), digit)) {
                    println!("{:?}",col_hash);
                    println!("col hash is false" );
                    
                    return false;
                } else {
                    col_hash.insert((col_idx.try_into().unwrap(), digit));
                }
                //  i think i got the math right, here. we have 9 boxes in 9x9
                //  so we divide rows and columns by 3 each and multiply one of them with 3 coz i consider the vec<hashset>
                // as  flattened matrix. for 3* (row/3) +(col/3)
                //* check in box_hash
                if (box_hash[3 * (row_idx / 3) + (col_idx / 3)].contains(&digit)) {
                    println!("box hash is false" );
                    return false;
                } else {
                    box_hash[3 * (row_idx / 3) + (col_idx / 3)].insert(digit);
                }
            }
        }
    }
    return true;
}

fn check_in_hash(hash: &HashSet<(u32, u32)>, number: (u32, u32)) -> bool {
    if hash.contains(&number) {
        return true;
    }
    return false;
}
