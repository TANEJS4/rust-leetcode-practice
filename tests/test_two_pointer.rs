use leetcode::two_pointer;

#[test]
fn test_valid_palindrome(){
    use two_pointer::valid_palindrome::valid_palindrome;
    assert!(valid_palindrome(String::from("A man, a plan, a canal: Panama")));
    assert!(!valid_palindrome(String::from("race a car")));
    assert!(valid_palindrome(String::from("")));
    assert!(valid_palindrome(String::from("a")));
}
#[test]
fn test_two_sum(){
    use two_pointer::two_sum::two_sum;
    assert_eq!(vec![1,2],two_sum(vec![2,7,11,15], 9));
    assert_eq!(vec![2,3], two_sum(vec![5, 25, 75], 100));    
}