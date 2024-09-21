pub fn bubble_sort<T: PartialOrd>(vector: &mut Vec<T>) {
  let size = vector.len();
  for i in 0..size {
    let mut swapped = false;

    for j in 0..(size - 1 - i) {
      if vector[j] > vector[j + 1] {
        vector.swap(j, j + 1);
        swapped = true;
      }
    }

    if !swapped {
      break;
    }
  }
}