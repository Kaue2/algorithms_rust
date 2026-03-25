mod bubble_sort;
mod merge_sort;
mod quick_sort;
mod radix_sort;

pub use self::bubble_sort::bubble_sort;
pub use self::merge_sort::top_down_merge_sort;
pub use self::quick_sort::quick_sort;
pub use self::radix_sort::radix_sort;

use std::cmp;
use std::collections::HashSet;

pub fn have_same_elements<T>(a: &[T], b: &[T]) -> bool
where
    T: cmp::PartialOrd + cmp::Eq + std::hash::Hash,
{
    use std::collections::HashSet;

    if a.len() == b.len() {
        let set_a: HashSet<&T> = a.iter().collect();
        let set_b: HashSet<&T> = b.iter().collect();
        set_a == set_b
    } else {
        false
    }
}

pub fn is_sorted<T>(arr: &[T]) -> bool
where
    T: cmp::PartialOrd,
{
    arr.windows(2).all(|w| w[0] <= w[1])
}

pub fn is_descending_sorted<T>(arr: &[T]) -> bool
where
    T: cmp::PartialOrd,
{
    arr.windows(2).all(|w| w[0] >= w[1])
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_sorted() {
        use super::*;

        assert!(is_sorted(&[] as &[isize]));
        assert!(is_sorted(&["a"]));
        assert!(is_sorted(&[1, 2, 3]));
        assert!(is_sorted(&[0, 1, 1]));

        assert!(!is_sorted(&[1, 0]));
        assert!(!is_sorted(&[2, 3, 1, -1, 5]));
    }
}
