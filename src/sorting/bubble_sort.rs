pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }

    let mut sorted = false;
    let n = arr.len();
    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::{have_same_elements, is_sorted};

    #[test]
    fn descending() {
        let mut vec1 = vec![6, 5, 4, 3, 2, 1];
        let cloned = vec1.clone();
        bubble_sort(&mut vec1);
        assert!(is_sorted(&vec1) && have_same_elements(&vec1, &cloned));
    }
}
