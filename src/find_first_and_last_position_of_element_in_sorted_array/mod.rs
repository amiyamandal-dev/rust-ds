use core::num;

use crate::sort::search::{self, binary_search};

pub struct Solution;

fn binary_search_temp(
    low: usize,
    high: usize,
    state_low: bool,
    nums: Vec<i32>,
    target: i32,
) -> i32 {
    if low > high {
        return -1_i32;
    } else {
        let mid = low + (high - low) / 2;
        if nums[mid] == target {
            if state_low == true {
                if mid != 0 && nums[mid - 1] == target {
                    return binary_search_temp(low, mid - 1, state_low, nums, target);
                }
                return mid as i32;
            } else {
                if mid != (nums.len() - 1) && nums[mid + 1] == target {
                    return binary_search_temp(mid + 1, high, state_low, nums, target);
                }
                return mid as i32;
            }
        } else if nums[mid] > target {
            if mid == 0 {
                return -1_i32;
            }
            return binary_search_temp(low, mid - 1, state_low, nums, target);
        } else {
            return binary_search_temp(mid + 1, high, state_low, nums, target);
        }
    }
}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() < 1 {
            return vec![-1, -1];
        }

        let high = binary_search_temp(0, nums.len() - 1, false, nums.clone(), target);
        let low = binary_search_temp(0, nums.len() - 1, true, nums, target);
        return vec![low, high];
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_code() {
        let rez = Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8);
        assert_eq!(rez, vec![3, 4]);
    }
    #[test]
    fn test_code_2() {
        let rez = Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6);
        assert_eq!(rez, vec![-1, -1]);
    }

    #[test]
    fn test_code_3() {
        let rez = Solution::search_range(vec![1], 0);
        assert_eq!(rez, vec![-1, -1]);
    }
}
