// Find max sum, w/o adding two adjacent elements.

// [0, 1, 0] -> maximizing # of added elements is not the goal, in this example, we just want the 2nd element for the max sum

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut cache = HashMap::new();
        return Self::rob_inner(&nums, 0, nums.len() - 1, &mut cache);
    }

    fn rob_inner(
        nums: &[i32],
        lb: usize,
        ub: usize,
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if let Some(cached_result) = cache.get(&(lb, ub)) {
            return *cached_result;
        }

        if lb > ub {
            return 0;
        } else if lb == ub {
            return nums[lb];
        } else {
            let result = vec![
                nums[lb + 1],
                nums[lb] + Self::rob_inner(nums, lb + 2, ub, cache),
                Self::rob_inner(nums, lb + 1, ub, cache),
            ]
            .into_iter()
            .max()
            .unwrap();

            cache.insert((lb, ub), result);

            return result;
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(Solution::rob(vec![2]), 2);
        assert_eq!(Solution::rob(vec![1, 2]), 2);
        assert_eq!(Solution::rob(vec![1, 5, 1]), 5);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![0, 20, 20, 0]), 20);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(Solution::rob(vec![10, 2, 3, 10]), 20);
        assert_eq!(
            Solution::rob(vec![
                228, 127, 13, 60, 76, 48, 270, 119, 86, 351, 130, 64, 205, 75, 374, 29, 112, 285,
                214, 298, 341, 164, 302, 338, 10, 382, 237, 82, 20, 338, 340, 93, 74, 334, 40, 97,
                241, 223, 130, 226, 26, 149, 8, 307, 175, 89, 353, 113, 302, 72, 120, 296, 122,
                118, 229, 301, 66, 300, 104, 56, 217, 334, 11, 356, 373, 55, 370, 75, 119, 166,
                192, 120, 317, 344, 396, 166, 278, 190, 21, 41, 306, 376, 367, 111, 83, 117, 382,
                205, 56, 19, 166, 243, 394, 259, 102, 158, 162, 37, 26, 199
            ]),
            11176
        );
        assert_eq!(
            Solution::rob(vec![
                310, 299, 274, 318, 277, 287, 397, 338, 17, 194, 248, 292, 199, 55, 23, 126, 51,
                168, 357, 209, 181, 340, 248, 31, 152, 305, 168, 70, 193, 162, 217, 156, 213, 186,
                251, 366, 93, 354, 50, 18, 166, 159, 0, 39, 181, 81, 253, 140, 89, 48, 86, 231,
                101, 146, 217, 110, 334, 217, 146, 375, 150, 70, 118, 305, 334, 238, 186, 243, 259,
                310, 375, 108, 113, 289, 224, 148, 195, 95, 235, 46, 377, 213, 288, 138, 159, 39,
                297, 120, 250, 234, 307, 367, 358, 235, 257, 152, 38, 297, 136, 82
            ]),
            10942
        );
    }
}
