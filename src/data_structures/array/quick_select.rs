/**
 * [QuickSelect](https://www.geeksforgeeks.org/quickselect-algorithm/) is an algorithm to find the kth smallest number
 *
 * Notes:
 * -QuickSelect is related to QuickSort, thus has optimal best and average
 * -case (O(n)) but unlikely poor worst case (O(n^2))
 * -This implementation uses randomly selected pivots for better performance
 *
 * @complexity: O(n) (on average )
 * @complexity: O(n^2) (worst case)
 * @flow
 */
pub fn quick_select(arr: Vec<i32>, k: usize) -> i32 {
    let arr_len = arr.len();
    let mut arr = arr.clone();
    _quick_select(&mut arr, 0, arr_len - 1, k)
}

fn _quick_select(arr: &mut Vec<i32>, l: usize, r: usize, k: usize) -> i32 {
    if k > 0 && k <= arr.len() {
        let index = partition(arr, l, r);

        if index - l == k - 1 {
            return arr[index];
        }

        if index - l > k - 1 {
            return _quick_select(arr, l, index - 1, k);
        }

        return _quick_select(arr, index + 1, r, k - index + l - 1);
    }

    panic!("Index out of bound");
}

fn partition(arr: &mut Vec<i32>, l: usize, r: usize) -> usize {
    let pivot = arr[r];
    let mut i = l;

    for j in l..r {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, r);

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_select() {
        assert_eq!(quick_select(vec![1, 2, 3, 4, 5], 2), 2);
        assert_eq!(quick_select(vec![0, 1, 5, 11, 4, 1, 2, 4, 9, 22, 23], 5), 4);
        assert_eq!(quick_select(vec![10, 4, 5, 8, 6, 11, 26], 3), 6);
        assert_eq!(quick_select(vec![3, 1, 8, 4, 2, 6, 7, 5, 9], 7), 7);
    }
}
