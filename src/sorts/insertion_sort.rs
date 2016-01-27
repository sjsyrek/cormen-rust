/// Borrow a mutable vector, perform an insertion sort of its contents, and return it.
///
/// # Example
///
/// ```
/// # use cormen_rust::*;
/// let mut a: Vec<isize> = vec![3, 2, 1];
/// insertion_sort(&mut a);
/// assert_eq!(a, [1, 2, 3]);
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