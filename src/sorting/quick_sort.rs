pub fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot = hi;
    let mut i = lo;
    let mut j = hi - 1;

    loop {
        while arr[i] < arr[pivot] {
            i += 1;
        }
        while j > 0 && arr[j] > arr[pivot] {
            j -= 1;
        }
        if j == 0 || i >= j {
            break;
        } else if arr[i] == arr[j] {
            i += 1;
            j -= 1;
        } else {
            arr.swap(i, j);
        }
    }
    arr.swap(i, pivot);
    i
}

fn _quick_sort<T: Ord>(arr: &mut [T], mut lo: usize, mut hi: usize) {
    while lo < hi {
        let pivot = partition(arr, lo, hi);

        if pivot - lo < hi - lo {
            // tail call optimization
            if pivot > 0 {
                _quick_sort(arr, lo, pivot - 1);
            }
            lo = pivot + 1;
        } else {
            _quick_sort(arr, pivot + 1, hi);
            hi = pivot - 1;
        }
    }
}

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len > 0 {
        _quick_sort(arr, 0, len - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn basic() {
        let mut arr = vec![-3, 2, 72, 3, -8, 9, 10];
        let cloned = arr.clone();
        quick_sort(&mut arr);

        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
}
