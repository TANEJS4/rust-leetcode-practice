use leetcode::two_pointer;

#[test]
fn test_valid_palindrome() {
    use two_pointer::valid_palindrome::valid_palindrome;
    assert!(valid_palindrome(String::from(
        "A man, a plan, a canal: Panama"
    )));
    assert!(!valid_palindrome(String::from("race a car")));
    assert!(valid_palindrome(String::from("")));
    assert!(valid_palindrome(String::from("a")));
}
#[test]
fn test_two_sum() {
    use two_pointer::two_sum::two_sum;
    assert_eq!(vec![1, 2], two_sum(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![2, 3], two_sum(vec![5, 25, 75], 100));
}
#[test]
fn test_three_sum() {
    use two_pointer::three_sum::three_sum;
    assert_eq!(
        vec![vec![-1, -1, 2], vec![0, -1, 1]],
        three_sum(vec![-1, 0, 1, 2, -1, -4])
    );
    assert_eq!(
        vec![vec![-2, 0, 2], vec![-2, 1, 1]],
        three_sum(vec![-2, 0, 1, 1, 2])
    );
    assert_eq!(
        vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        three_sum(vec![-1, 0, 1, 2, -1, -4])
    );

    assert_eq!(
        vec![
            vec![-4, 0, 4],
            vec![-4, 1, 3],
            vec![-3, -1, 4],
            vec![-3, 0, 3],
            vec![-3, 1, 2],
            vec![-2, -1, 3],
            vec![-2, 0, 2],
            vec![-1, -1, 2],
            vec![-1, 0, 1]
        ],
        three_sum(vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4])
    );
    assert_eq!(
        vec![
            vec![-15, 1, 14],
            vec![-15, 2, 13],
            vec![-15, 3, 12],
            vec![-15, 4, 11],
            vec![-15, 5, 10],
            vec![-15, 6, 9],
            vec![-15, 7, 8],
            vec![-14, 0, 14],
            vec![-14, 1, 13],
            vec![-14, 2, 12],
            vec![-14, 3, 11],
            vec![-14, 4, 10],
            vec![-14, 5, 9],
            vec![-14, 6, 8],
            vec![-14, 7, 7],
            vec![-13, -1, 14],
            vec![-13, 0, 13],
            vec![-13, 1, 12],
            vec![-13, 2, 11],
            vec![-13, 3, 10],
            vec![-13, 4, 9],
            vec![-13, 5, 8],
            vec![-13, 6, 7],
            vec![-12, -2, 14],
            vec![-12, -1, 13],
            vec![-12, 0, 12],
            vec![-12, 1, 11],
            vec![-12, 2, 10],
            vec![-12, 3, 9],
            vec![-12, 4, 8],
            vec![-12, 5, 7],
            vec![-12, 6, 6],
            vec![-11, -3, 14],
            vec![-11, -2, 13],
            vec![-11, -1, 12],
            vec![-11, 0, 11],
            vec![-11, 1, 10],
            vec![-11, 2, 9],
            vec![-11, 3, 8],
            vec![-11, 4, 7],
            vec![-11, 5, 6],
            vec![-10, -4, 14],
            vec![-10, -3, 13],
            vec![-10, -2, 12],
            vec![-10, -1, 11],
            vec![-10, 0, 10],
            vec![-10, 1, 9],
            vec![-10, 2, 8],
            vec![-10, 3, 7],
            vec![-10, 4, 6],
            vec![-10, 5, 5],
            vec![-9, -5, 14],
            vec![-9, -4, 13],
            vec![-9, -3, 12],
            vec![-9, -2, 11],
            vec![-9, -1, 10],
            vec![-9, 0, 9],
            vec![-9, 1, 8],
            vec![-9, 2, 7],
            vec![-9, 3, 6],
            vec![-9, 4, 5],
            vec![-8, -6, 14],
            vec![-8, -5, 13],
            vec![-8, -4, 12],
            vec![-8, -3, 11],
            vec![-8, -2, 10],
            vec![-8, -1, 9],
            vec![-8, 0, 8],
            vec![-8, 1, 7],
            vec![-8, 2, 6],
            vec![-8, 3, 5],
            vec![-8, 4, 4],
            vec![-7, -7, 14],
            vec![-7, -6, 13],
            vec![-7, -5, 12],
            vec![-7, -4, 11],
            vec![-7, -3, 10],
            vec![-7, -2, 9],
            vec![-7, -1, 8],
            vec![-7, 0, 7],
            vec![-7, 1, 6],
            vec![-7, 2, 5],
            vec![-7, 3, 4],
            vec![-6, -5, 11],
            vec![-6, -4, 10],
            vec![-6, -3, 9],
            vec![-6, -2, 8],
            vec![-6, -1, 7],
            vec![-6, 0, 6],
            vec![-6, 1, 5],
            vec![-6, 2, 4],
            vec![-6, 3, 3],
            vec![-5, -5, 10],
            vec![-5, -4, 9],
            vec![-5, -3, 8],
            vec![-5, -2, 7],
            vec![-5, -1, 6],
            vec![-5, 0, 5],
            vec![-5, 1, 4],
            vec![-5, 2, 3],
            vec![-4, -4, 8],
            vec![-4, -3, 7],
            vec![-4, -2, 6],
            vec![-4, -1, 5],
            vec![-4, 0, 4],
            vec![-4, 1, 3],
            vec![-4, 2, 2],
            vec![-3, -3, 6],
            vec![-3, -2, 5],
            vec![-3, -1, 4],
            vec![-3, 0, 3],
            vec![-3, 1, 2],
            vec![-2, -2, 4],
            vec![-2, -1, 3],
            vec![-2, 0, 2],
            vec![-2, 1, 1],
            vec![-1, -1, 2],
            vec![-1, 0, 1],
            vec![0, 0, 0]
        ],
        three_sum(vec![
            14, 4, 6, -1, 10, 9, -8, 7, -13, 14, -13, -11, -8, -9, 11, 14, -8, -14, -13, 7, -10,
            -15, -13, -11, -11, 11, 14, 13, 2, -14, 1, -7, -2, 14, -1, -15, 9, 7, -1, 3, 6, 1, 7,
            5, -1, -5, 4, -2, -4, -1, -9, -7, -1, -7, -11, 3, 12, 10, -7, -1, 12, 1, 8, -13, 1, 14,
            9, -13, 6, -7, -3, -11, 2, -11, 10, -14, -1, -9, 0, 2, 5, 6, 3, -11, 6, 7, 0, 3, 3, 0,
            -12, -8, -13, 3, -14, -5, 2, 10, -11, -14, -12, 1, -10, 5, 5, 7, -1, 11, 14, 6, -10,
            -4, -3, 8, -7, 10, 1, 8, -1, -11, -15, -6, -12, -13, 12, -11
        ])
    )
}

#[test]
fn test_max_area (){
use two_pointer::max_area::max_area;
assert_eq!(49, max_area(Vec::from([1,8,6,2,5,4,8,3,7])));
assert_eq!(1, max_area(Vec::from([1,1])));
}

#[test]
fn test_trap(){
	use two_pointer::trap::trap;
    assert_eq!(9, trap(Vec::from([4,2,0,3,2,5])));
    assert_eq!(6,trap(Vec::from([0,1,0,2,1,0,1,3,2,1,2,1])));
} 
