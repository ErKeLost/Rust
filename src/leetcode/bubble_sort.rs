// Desc: 冒泡排序 传入一个 arr 是mut 可变的 vec 引用 i32
// vec![64, 34, 90, 25, 12, 22, 11];
pub fn bubble_sort(arr: &mut Vec<i32>) {
    // 获取数组的长度 开始循环数组的len
    let len = arr.len();
    for i in 0..len {
        println!("--------------------");
        for j in 0..len - i - 1 {
            // 如果前一个元素大于后一个元素 则交换位置
            println!("arr[j]: {}, arr[j + 1]: {}", arr[j], arr[j + 1]);
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![64, 34, 90, 25, 12, 22, 11];
        bubble_sort(&mut arr);
        assert_eq!(arr, [11, 12, 22, 25, 34, 64, 90]);

        // let mut arr2 = vec![5, 4, 3, 2, 1];
        // bubble_sort(&mut arr2);
        // assert_eq!(arr2, [1, 2, 3, 4, 5]);
    }
}

// 冒泡排序就是 通过多次遍历 未排序的数组 元素 将大的元素或者小的元素冒泡的数组的最后或者最前 直到整个数组有序
