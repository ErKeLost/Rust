pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut nums = nums;

    nums.sort(); // 先将数组排序

    for i in 0..nums.len() - 2 {
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
        let nums2 = vec![0, 0, 0];
        let result2 = three_sum(nums2);
        assert_eq!(result2, vec![vec![0, 0, 0]]);

        // 添加更多测试用例...
    }
}
