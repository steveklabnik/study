extern crate rand;
extern crate num;
use num::traits::Bounded;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    return;
}

fn selection_sort<T: Ord + Clone>(numbers: Vec<T>) -> Vec<T> {
    let mut numbers: Vec<T> = numbers.clone();

    for i in 1..numbers.len() {
        let mut min = i;

        for j in (i + 1)..numbers.len() {
            if numbers[j] < numbers[min] {
                min = j;
            }
        }
        numbers.swap(min, i);
    }

    numbers
}

fn merge_sort<T: Ord + Bounded + Clone>(mut arr: Vec<T>) -> Vec<T> {
    if arr.len() < 2 { 
        return arr;
    }
    if arr.len() == 2 {
        if arr[0] > arr[1] { 
            arr.swap(0,1);
        }
        return arr;
    }

    let (left, right) = arr.split_at(arr.len()/2);

    merge(
        merge_sort(left.to_vec()),
        merge_sort(right.to_vec())
    )
}

fn merge<T: Ord + Bounded + Clone>(left: Vec<T>, right: Vec<T>) -> Vec<T>{
    let mut merged = vec![];
    let mut left = left.clone();
    left.push(Bounded::max_value());

    let mut right = right.clone();
    right.push(Bounded::max_value());

    let mut l=0;
    let mut r=0;

    while merged.len() < (left.len() - 1) + (right.len() - 1) {
        if left[l] < right[r] {
            merged.push(left[l].clone());
            l = l + 1;
        } else {
            merged.push(right[r].clone());
            r = r + 1;
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::merge_sort;
    use super::selection_sort;
    use rand::{self, Rng};

    #[test]
    fn merge() {
        let mut unsorted: Vec<usize> = vec![];
        let mut rng = rand::thread_rng();

        for _ in 1..99 {
            unsorted.push(rng.gen::<usize>());
        }

        let mut to_sorted = unsorted.clone();

        let sorted = merge_sort(unsorted);
        to_sorted.sort();

        assert_eq!(to_sorted, sorted);
    }

    #[test]
    fn selection() {
        let unsorted = vec![ 5, 15, 20, 13 ];
        let sorted = selection_sort(unsorted);

        assert_eq!(sorted, [5, 13, 15, 20]);
    }
}
