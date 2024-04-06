mod arrays_and_hash;

fn main() {

}
fn test_top_k_freq_elements(){
    use arrays_and_hash::top_k_freq_elements::top_k_freq_elements;
    println!("{:?}",top_k_freq_elements(vec![1,2,4,4,2,4,5,1], 2));
}

fn test_contains_duplicate(){
    use arrays_and_hash::contains_duplicate::contains_duplicate;
    let input = vec![1, 2, 3, 1];
    println!("{:?}",contains_duplicate(input));
}
fn test_group_anagrams(){
    use arrays_and_hash::group_anagrams::group_anagrams;
    println!("{:?}", group_anagrams(vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()]))
    
}
fn test_valid_anagram(){
    use arrays_and_hash::valid_anagram::valid_anagram;
    println!("{:?}",valid_anagram("hello".to_string(), "world".to_string()));
}
fn test_two_sum(){
    use arrays_and_hash::twosum::two_sum;
    println!("{:?}", two_sum(vec![3,2,4],6))
}