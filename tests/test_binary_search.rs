
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

#[test]
fn test_min_eating_speed(){
    use binary_search::min_eating_speed::min_eating_speed;
	
    assert_eq!(4,min_eating_speed(vec![3,6,7,11], 8));
    assert_eq!(30,min_eating_speed(vec![30,11,23,4,20], 5));
    assert_eq!(500000000,min_eating_speed(vec![1000000000], 2));
    assert_eq!(3,min_eating_speed(vec![805306368,805306368,805306368], 1000000000));
    assert_eq!(2,min_eating_speed(vec![312884470], 312884469));
} 


#[test]
fn test_find_min(){
	use binary_search::find_min::find_min;
	
} 
