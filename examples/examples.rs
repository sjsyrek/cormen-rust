extern crate cormen_rust;
use cormen_rust::*;

fn main() {
    // Insertion Sort
    let mut a: Vec<isize> = vec![5, 4, 3, 2, 1, -1, -2, -3, -4, -5];
    println!("Insertion sort of {}: {}", vec_to_str(&a), vec_to_str(insertion_sort(&mut a)));

    // Selection Sort
    let mut a: Vec<isize> = vec![5, 4, 3, 2, 1, -1, -2, -3, -4, -5];
    println!("Selection sort of {}: {}", vec_to_str(&a), vec_to_str(selection_sort(&mut a)));
}