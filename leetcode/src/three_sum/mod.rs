/**
 * [15] 3Sum
 *
 * Given an integer array nums, return all the triplets [nums[i], nums[j],
 * nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
 *
 * Notice that the solution set must not contain duplicate triplets.
 *
 * Example:
 *
 *
 * Input: nums = [-1,0,1,2,-1,-4]
 * Output: [[-1,-1,2],[-1,0,1]].
 *
 * Problem: https://leetcode.com/problems/3sum/
 */

struct Solution;

fn align_left(values: &Vec<i32>, left: usize) -> usize {
    let mut ret = left + 1;
    while ret < values.len() && values[ret] == values[ret - 1] {
        // to avoid outputting duplicates we keep incrementing our index
        // if we encounter the same element
        ret += 1;
    }
    ret
}

fn align_right(values: &Vec<i32>, right: usize, start: usize) -> usize {
    let mut ret = right - 1;
    while ret > start && values[ret] == values[ret + 1] {
        // to avoid outputting duplicates we keep decrementing our index
        // if we encounter the same element
        ret -= 1;
    }
    ret
}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted = nums.clone();
        sorted.sort();

        let mut ret = Vec::new();

        for (start, elem) in sorted.iter().enumerate() {
            if start > 0 && sorted[start - 1] == *elem {
                continue;
            }

            let mut left = start + 1;
            let mut right = sorted.len() - 1;

            while left < right {
                if sorted[left] + sorted[right] == -elem {
                    ret.push(vec![*elem, sorted[left], sorted[right]]);
                    left = align_left(&sorted, left);
                    right = align_right(&sorted, right, start);
                } else if sorted[left] + sorted[right] > -elem {
                    right = align_right(&sorted, right, start);
                } else {
                    left = align_left(&sorted, left);
                }
            }
        }

        ret
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        [[-1, -1, 2], [-1, 0, 1]]
    );
}
