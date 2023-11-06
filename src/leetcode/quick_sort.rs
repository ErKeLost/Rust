// pub fn quick_sort(arr: &mut Vec<i32>) {
//     let len = arr.len();
//     if len < 2 {
//         return; // 基准情况：空数组或只有一个元素，无需排序
//     }

//     let pivot_index = partition(arr);

//     // 递归地对分区的两侧进行排序
//     quick_sort(&mut arr[0..pivot_index].to_vec());
//     quick_sort(&mut arr[pivot_index + 1..].to_vec());
// }

// fn partition(arr: &mut Vec<i32>) -> usize {
//     let len = arr.len();
//     let pivot_index = len / 2;

//     let mut i = 0;
//     for j in 0..len - 1 {
//         if arr[j] <= arr[len - 1] {
//             arr.swap(i, j);
//             i += 1;
//         }
//     }

//     arr.swap(i, len - 1);
//     i
// }

pub fn quick_sort(vector: &mut Vec<i32>) {
    quick_sort_inside(vector, 0, vector.len());
}

fn quick_sort_inside(vector: &mut Vec<i32>, left: usize, right: usize) {
    // 当需要排序的分区长度小于 2 时就可以结束递归了
    if right - left < 2 {
        return;
    }
    // 对数列的 [left, right) 部分进行排序，并返回第一个大于或者等于基准数所在的位置
    let p = partition(vector, left, right);
    // 对数列的 [left, p) 部分进行排序
    quick_sort_inside(vector, left, p);
    // 对数列的 [p, right) 部分进行排序
    quick_sort_inside(vector, p, right);
}

fn partition(vector: &mut Vec<i32>, p: usize, l: usize) -> usize {
    // 选取基准值（这里可以进行优化，使用随机数或者三数取中等）
    let pivot = vector[(l + p) / 2];
    // 对数列进行交互时需要使用到的中间变量
    let mut temp: i32;
    // 比基准值小的数的当前指针
    let mut prev = p;
    // 比基准值大的数的当前指针
    let mut last = l - 1;

    loop {
        // 将左指针移动到比基准值大的数
        while vector[prev] < pivot && prev < last {
            prev += 1;
        }
        // 将右指针移动到比基准值小的数
        while vector[last] > pivot && prev < last {
            last -= 1;
        }
        // 如果移动后，左指针大于等于右指针了，则表示当前分区已经排序完成了
        if prev >= last {
            break;
        }
        // 交换当前的大数与小数
        temp = vector[last];
        vector[last] = vector[prev];
        vector[prev] = temp;
        // 使当前的前指针向后移动一位
        prev += 1;
    }
    return prev;
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
