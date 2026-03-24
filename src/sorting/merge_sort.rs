fn merge<T: Ord + Copy>(arr: &mut [T], mid: usize) {
    let left_half = arr[..mid].to_vec();
    let right_half = arr[mid..].to_vec();

    let mut l = 0;
    let mut r = 0;

    for v in arr {
        if r == right_half.len() || (l < left_half.len() && left_half[l] < right_half[r]) {
            *v = left_half[l];
            l += 1;
        } else {
            *v = right_half[r];
            r += 1;
        }
    }
}

pub fn top_down_merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;

        top_down_merge_sort(&mut arr[..mid]); // passando um slice do array
        top_down_merge_sort(&mut arr[mid..]);

        merge(arr, mid);
    }
}

#[cfg(test)]
mod tests {
    use super::top_down_merge_sort;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn basic() {
        let mut arr = vec![10, 3, 7, 12, -1, 3, -4];
        let clonned = arr.clone();

        top_down_merge_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &clonned));
    }
}
