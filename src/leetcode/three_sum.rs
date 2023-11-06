// pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
//     let mut result: Vec<Vec<i32>> = Vec::new();
//     let mut nums = nums;
//     println!("nums: {:?}", nums);
//     nums.sort(); // 先将数组排序
//     println!("nums: {:?}", nums);
//     for i in 0..nums.len() - 2 {
//         if i == 0 || (i > 0 && nums[i] != nums[i - 1]) {
//             let mut left = i + 1;
//             let mut right = nums.len() - 1;
//             let target = 0 - nums[i];

//             while left < right {
//                 if nums[left] + nums[right] == target {
//                     result.push(vec![nums[i], nums[left], nums[right]]);
//                     while left < right && nums[left] == nums[left + 1] {
//                         left += 1;
//                     }
//                     while left < right && nums[right] == nums[right - 1] {
//                         right -= 1;
//                     }
//                     left += 1;
//                     right -= 1;
//                 } else if nums[left] + nums[right] < target {
//                     left += 1;
//                 } else {
//                     right -= 1;
//                 }
//             }
//         }
//     }

//     result
// }

// let nums1 = vec![-1, 0, 1, 2, -1, -4];
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    // to_vec 表示返回一个新的 Vec clone
    let mut nums = nums.to_vec();
    // nums.sort(); // 先将数组排序
    // 使用不稳定排序
    nums.sort_unstable();
    // 循环迭代数组 长度是 0 到 num 的length - 2 因为算法至少需要三个元素来形成三元组
    for i in 0..nums.len() - 2 {
        // 这个条件检查当前元素 nums[i] 是否与前一个元素不同，或者是数组的第一个元素
        // 。这是为了处理重复元素的情况，以确保不会有相同的三元组被多次添加到结果中。
        if i == 0 || (i > 0 && nums[i] != nums[i - 1]) {
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            let target = 0 - nums[i];

            while left < right {
                if nums[left] + nums[right] == target {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                } else if nums[left] + nums[right] < target {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_three_sum() {
        // 测试用例 1
        let nums1 = vec![-1, 0, 1, 2, -1, -4];
        let result1 = three_sum(nums1);
        assert_eq!(result1, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);

        // 测试用例 2
        // let nums2 = vec![0, 0, 0];
        // let result2 = three_sum(nums2);
        // assert_eq!(result2, vec![vec![0, 0, 0]]);

        // 添加更多测试用例...
    }
}
