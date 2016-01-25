//! Rust implementations of algorithms from
//! Introduction to Algorithms by Thomas Cormen, et al.

#![crate_name = "cormen_rust"]
#![crate_type = "lib"]

/// Convert a vector of type isize to a String for pretty printing.
/// 
/// # Example
///
/// ```
/// let mut v: Vec<isize> = vec![1, 2, 3];
/// let s = "[1, 2, 3]".to_string();
/// assert_eq!(vec_to_str(&v), s);
/// # fn vec_to_str(v: &Vec<isize>) -> String {
/// #    let s: String = v.iter()
/// #                     .map(|x| x.to_string() + ", ")
/// #                     .collect();
/// #    "[".to_string() + &s.trim_right_matches(", ").to_string() + "]"
/// # }
/// ```
pub fn vec_to_str(v: &Vec<isize>) -> String {
    let s: String = v.iter()
                     .map(|x| x.to_string() + ", ")
                     .collect();
    "[".to_string() + &s.trim_right_matches(", ").to_string() + "]"
}

/// Borrow a mutable vector, perform an insertion sort of its contents, and return it.
///
/// # Example
///
/// ```
/// let mut a: Vec<isize> = vec![3, 2, 1];
/// insertion_sort(&mut a);
/// assert_eq!(a, [1, 2, 3]);
/// # fn insertion_sort(a: &mut Vec<isize>) -> &mut Vec<isize> {
/// #    for j in 1..a.len() {
/// #        let key = a[j];
/// #        let mut i = j;
/// #        while i > 0 && a[i - 1] > key {
/// #            a[i] = a[i - 1];
/// #            i = i - 1;
/// #        }
/// #        a[i] = key;
/// #    }
/// #    a
/// # }
/// ```
pub fn insertion_sort(a: &mut Vec<isize>) -> &mut Vec<isize> {
    for j in 1..a.len() { // In Cormen, this loop starts at index 2 for 1..n arrays
        let key = a[j];
        // Insert a[j] into the sorted sequence a[0..j - 1]
        let mut i = j;
        while i > 0 && a[i - 1] > key { // since i is type usize, it can't ever equal -1
            a[i] = a[i - 1];
            i = i - 1;
        }
        a[i] = key;
    }
    a // return the borrowed vector
}

/// Sort a vector using the selection sort algorithm.
/// 
/// # Example
/// 
/// ```
/// let mut a: Vec<isize> = vec![3, 2, 1];
/// selection_sort(&mut a);
/// assert_eq!(a, [1, 2, 3]);
/// # fn selection_sort(a: &mut Vec<isize>) -> &mut Vec<isize> {
/// #     let n = a.len();
/// #     for j in 0..n {
/// #         let mut smallest = j;
/// #         for i in j + 1..n {
/// #             if a[i] < a[smallest] {
/// #                 smallest = i;
/// #             }
/// #         }
/// #         let temp = a[j];
/// #         a[j] = a[smallest];
/// #         a[smallest] = temp;
/// #    }
/// #     a // return the borrowed vector
/// # }
/// ```
pub fn selection_sort(a: &mut Vec<isize>) -> &mut Vec<isize> {
    let n = a.len();
    for j in 0..n {
        let mut smallest = j;
        for i in j + 1..n {
            if a[i] < a[smallest] {
                smallest = i;
            }
        }
        let temp = a[j];
        a[j] = a[smallest];
        a[smallest] = temp;
    }
    a // return the borrowed vector
}
