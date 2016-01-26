extern crate cormen_rust;
use cormen_rust::*;

fn main() {
    // Insertion Sort
    let mut a: Vec<isize> = vec![5, 4, 3, 2, 1, -1, -2, -3, -4, -5];
    println!("Insertion sort of {}: {}", vec_to_str(&a), vec_to_str(insertion_sort(&mut a)));

    // Selection Sort
    let mut a: Vec<isize> = vec![50, 40, 30, 20, 10, -10, -20, -30, -40, -50];
    println!("Selection sort of {}: {}", vec_to_str(&a), vec_to_str(selection_sort(&mut a)));

    // Merge Sort
    let mut a: Vec<isize> = vec![15, 14, 13, 12, 11, -11, -12, -13, -14, -15];
    let p = 0;
    let r = a.len();
    println!("Merge sort of {}: {}", vec_to_str(&a), vec_to_str(merge_sort(&mut a, p, r)));
}