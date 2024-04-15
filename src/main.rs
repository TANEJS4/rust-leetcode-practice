mod arrays_and_hash;

fn main() {
    // print!("{:}", 8/3)
    test_longest_consecutive()
}
#[allow(dead_code)]
fn test_longest_consecutive(){
    use arrays_and_hash::longest_consecutive::longest_consecutive;
    assert_eq!(6, longest_consecutive(vec![100,4,200,1,3,2, 6,7,8,9,10,11]));
    assert_eq!(4, longest_consecutive(vec![100,1,2,3,50,4]));
    assert_eq!(1,longest_consecutive(vec![1]));
    assert_eq!(0,longest_consecutive(vec![]));
    assert_eq!(9,longest_consecutive(vec![0, 3,7,2,5,8,4,6,0,1]));
    assert_eq!(3,longest_consecutive(vec![1,2,0,1]));
    assert_eq!(1,longest_consecutive(vec![0,0]));
    }
#[allow(dead_code)]
fn test_valid_sudoku() {
    use arrays_and_hash::valid_sudoku::valid_sudoku;
    println!(
        "{:?}",
        valid_sudoku(vec![
            vec!['.', '8', '7', '6', '5', '4', '3', '2', '1'],
            vec!['2', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['3', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['4', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['5', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['6', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['7', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['8', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['9', '.', '.', '.', '.', '.', '.', '.', '.']
        ])
    )
}

#[allow(dead_code)]
fn test_product_except_self() {
    use arrays_and_hash::product_except_self::product_except_self;
    println!("{:?}", product_except_self(vec![1, 2, 3, 4]));
}
#[allow(dead_code)]
fn test_top_k_freq_elements() {
    use arrays_and_hash::top_k_freq_elements::top_k_freq_elements;
    println!("{:?}", top_k_freq_elements(vec![1, 2, 4, 4, 2, 4, 5, 1], 2));
}
#[allow(dead_code)]
fn test_contains_duplicate() {
    use arrays_and_hash::contains_duplicate::contains_duplicate;
    let input = vec![1, 2, 3, 1];
    println!("{:?}", contains_duplicate(input));
}
#[allow(dead_code)]
fn test_group_anagrams() {
    use arrays_and_hash::group_anagrams::group_anagrams;
    println!(
        "{:?}",
        group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string()
        ])
    )
}
#[allow(dead_code)]
fn test_valid_anagram() {
    use arrays_and_hash::valid_anagram::valid_anagram;
    println!(
        "{:?}",
        valid_anagram("hello".to_string(), "world".to_string())
    );
}
#[allow(dead_code)]
fn test_two_sum() {
    use arrays_and_hash::twosum::two_sum;
    println!("{:?}", two_sum(vec![3, 2, 4], 6))
}
