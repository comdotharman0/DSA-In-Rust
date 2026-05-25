use std::cmp::{Eq,PartialOrd};
pub struct MergeSort;

impl MergeSort{
pub fn merge(left: Vec<usize>, right: Vec<usize>) -> Vec<usize> {
    let mut result = vec![];
    let (mut i, mut j) = (0, 0);
    
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }
    
    // Safely append whatever is left over using extend
    result.extend(&left[i..]);
    result.extend(&right[j..]);
    
    result
}

pub fn merge_sort(mut a: Vec<usize>) -> Vec<usize> {
    if a.len() <= 1 {
        return a;
    }
    
    let mid = a.len() / 2;
    let left_half = &a[..mid];
    let right_half = &a[mid..];

    let sorted_left = Self:: merge_sort((*left_half).to_vec());
    let sorted_right = Self::merge_sort((*right_half).to_vec());

    Self::merge(sorted_left, sorted_right)
}


}


