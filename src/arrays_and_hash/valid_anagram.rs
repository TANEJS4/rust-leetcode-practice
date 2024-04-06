pub fn valid_anagram(s: String, t: String) -> bool
{
    //create vec out of string
    let mut svec: Vec<char> = s.chars().collect();
    svec.sort();
    let mut tvec: Vec<char> = t.chars().collect();
    tvec.sort();
    if svec==tvec{
        return true;
    }
    return false;
}
