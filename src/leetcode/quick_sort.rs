pub fn quick_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    if len < 2 {
        return; // 基准情况：空数组或只有一个元素，无需排序
    }

    let pivot_index = partition(arr);

    // 递归地对分区的两侧进行排序
    quick_sort(&mut arr[0..pivot_index].to_vec());
    quick_sort(&mut arr[pivot_index + 1..].to_vec());
}

fn partition(arr: &mut Vec<i32>) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;

    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_quick_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_quick_sort_single() {
        let mut arr = vec![42];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }
}
