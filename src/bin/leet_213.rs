// 213 House Robber II
//
// You are a professional robber planning to rob houses along a street. Each house has a certain
// amount of money stashed. All houses at this place are arranged in a circle. That means the first
// house is the neighbor of the last one. Meanwhile, adjacent houses have a security system
// connected, and it will automatically contact the police if two adjacent houses were broken into
// on the same night.
//
// Given an integer array nums representing the amount of money of each house, return the maximum
// amount of money you can rob tonight without alerting the police.

struct Solution {}

use std::collections::HashMap;

impl Solution {
    // Tweak solution for 198 to account for the "ring buffer" instead of the vector. Return
    // max(rob(v[1], ..., v[n]), rob(v[0], ..., v[n-1])), i.e., ensure that only either v[0] or v[n]
    // appear in any valid solution, but never both.
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        if nums.len() == 1 {
            return nums[0];
        }

        let last = nums.pop().unwrap();
        let result1 = Self::rob_inner(&nums, 0, &mut HashMap::new());
        nums.push(last);
        nums.remove(0);
        let result2 = Self::rob_inner(&nums, 0, &mut HashMap::new());
        return result2.max(result1);
    }

    fn rob_inner(nums: &[i32], lb: usize, cache: &mut HashMap<usize, i32>) -> i32 {
        if let Some(cached_result) = cache.get(&lb) {
            return *cached_result;
        }

        if lb > nums.len() - 1 {
            return 0;
        } else if lb == nums.len() - 1 {
            return nums[lb];
        } else {
            let result = vec![
                nums[lb + 1],
                nums[lb] + Self::rob_inner(nums, lb + 2, cache),
                Self::rob_inner(nums, lb + 1, cache),
            ]
            .into_iter()
            .max()
            .unwrap();

            cache.insert(lb, result);

            return result;
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn test_rob() {
        let testcases = vec![
            (vec![0], 0),
            (vec![2, 3, 2], 3),
            (vec![1, 2, 3, 1], 4),
            (vec![1, 2, 3], 3),
        ];
        for (nums, result) in testcases {
            if Solution::rob(nums.clone()) != result {
                println!("{:?}", nums);
                panic!();
            }
        }
    }
}
