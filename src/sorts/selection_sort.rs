/// Sort a vector using the selection sort algorithm.
/// 
/// # Example
/// 
/// ```
/// # use cormen_rust::*;
/// let mut a: Vec<isize> = vec![3, 2, 1];
/// selection_sort(&mut a);
/// assert_eq!(a, [1, 2, 3]);
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