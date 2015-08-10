fn main() { return; }

#[test]
fn test_merge_sort() {
  let unsorted = vec![ 5, 15, 20, 13 ];
  let sorted = selection_sort(unsorted);

  assert_eq!(sorted, [5, 13, 15, 20]);
}

#[test]
fn test_selection_sort() {
  let unsorted = vec![ 5, 15, 20, 13 ];
  let sorted = merge_sort(unsorted);

  assert_eq!(sorted, [5, 13, 15, 20]);
}

fn selection_sort<T: Ord+Clone>(numbers: Vec<T>) -> Vec<T> {
  let mut numbers: Vec<T> = numbers.clone();
  for i in 1..numbers.len() {
      let mut min = i;
      for j in i+1..numbers.len() {
          if numbers[j] < numbers[min] {
              min = j;
          }
      }

    numbers.swap(min, i);
  }
  return numbers;
}

fn merge_sort<T: Ord+Clone>(mut arr: Vec<T>) -> Vec<T> {
  if arr.len()<2 { return arr };
  if arr.len()==2 {
      if arr[0] > arr[1] { arr.swap(0,1); }
      return arr;
  }

  let (left, right) = arr.split_at(arr.len()/2);
  return merge(
    merge_sort(left.to_vec()),
    merge_sort(right.to_vec())
  );
}

fn merge<T: Ord+Clone>(left: Vec<T>, right: Vec<T>) -> Vec<T>{
  let mut merged = vec![];
  let mut j=0;
  let mut k=0;
  while merged.len() < left.len()+right.len() {
    if left[j] < right[k] {
        merged.push(left[j].clone());
        j=j+1;
    } else {
        merged.push(right[k].clone());
        k=k+1;
    }
  }
  return merged;
}
