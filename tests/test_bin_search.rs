use leetcode::binary_search;

#[test]
fn test_search(){
	use binary_search::search::search;
    assert_eq!(5,search(vec![-1,0,1, 3,5,9,12, 14, ],9));
} 




#[test]
fn test_search_matrix(){
	use binary_search::search_matrix::search_matrix;
    
    assert!(!search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 2));
    assert!(!search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 12));
    assert!(search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 23));
    assert!(!search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 100));
    assert!(!search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 0));
    assert!(search_matrix(vec![vec![1,3,5,7]], 3));
} 
