/// Merge two sorted subvectors into a single, sorted vector. An auxiliary function of merge_sort.
/// 
/// # Example
/// 
/// ```
/// # use cormen_rust::*;
/// let mut a: Vec<isize> = vec![1, 3, 5, 2, 4, 6];
/// merge(&mut a, 0, 3, 6);
/// assert_eq!(a, [1, 2, 3, 4, 5, 6]);
/// ```
pub fn merge(a: &mut Vec<isize>, p: usize, q: usize, r: usize) -> &mut Vec<isize> {
    let n1 = q - p;
    let n2 = r - q;
    let mut left: Vec<isize> = vec![0; (n1 + 1)];
    let mut right: Vec<isize> = vec![0; (n2 + 1)];
    for i in 0..n1 {
        left[i] = a[p + i];
    }
    for j in 0..n2 {
        right[j] = a[q + j];
    }
    left[n1] = isize::max_value();
    right[n2] = isize::max_value();
    let mut i = 0;
    let mut j = 0;
    for k in p..r {
        if left[i] <= right[j] {
            a[k] = left[i];
            i += 1;
        } else {
            a[k] = right[j];
            j += 1;
        }
    }
    a
}

/// Sort a vector using the merge sort algorithm.
/// 
/// # Example
/// 
/// ```
/// # use cormen_rust::*;
/// let mut a: Vec<isize> = vec![15, 14, 13, 12, 11, -11, -12, -13, -14, -15];
/// let p = 0;
/// let r = a.len();
/// merge_sort(&mut a, p, r);
/// assert_eq!(a, [-15, -14, -13, -12, -11, 11, 12, 13, 14, 15]);
/// ```
pub fn merge_sort(a: &mut Vec<isize>, p: usize, r: usize) -> &mut Vec<isize> {
    if r - p > 1 {
        let q = (p + r) / 2;
        merge_sort(a, p, q);
        merge_sort(a, q, r);
        merge(a, p, q, r);
    }
    a
}