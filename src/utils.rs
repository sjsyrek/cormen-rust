/// Convert a vector of type isize to a String for pretty printing.
/// 
/// # Example
///
/// ```
/// # use cormen_rust::*;
/// let mut v: Vec<isize> = vec![1, 2, 3];
/// let s = "[1, 2, 3]".to_string();
/// assert_eq!(vec_to_str(&v), s);
/// ```
pub fn vec_to_str(v: &Vec<isize>) -> String {
    let s: String = v.iter()
                     .map(|x| x.to_string() + ", ")
                     .collect();
    "[".to_string() + &s.trim_right_matches(", ").to_string() + "]"
}