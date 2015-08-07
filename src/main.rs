fn main() { return; }

#[test]
fn trivial_sort() {
  let unsorted = vec![ 5, 15, 20, 13 ];
  let sorted = selection_sort(unsorted);

  assert_eq!(sorted, [5, 13, 15, 20]);
}

fn selection_sort<T: Ord+Clone>(numbers: Vec<T>) -> Vec<T> {
  let mut sorted: Vec<T> = numbers.clone();
  for i in 1..sorted.len() {
      let mut min = i;
      for j in i+1..sorted.len() {
          if sorted[j] < sorted[min] {
              min = j;
          }
      }

    sorted.swap(min, i);
  }
  return sorted;
}
