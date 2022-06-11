/**
 * [704] Binary Search
 *
 * Given an array of integers nums which is sorted in ascending order, and an integer
 * target, write a function to search target in nums. If target exists, then return its
 * index. Otherwise, return -1.
 *
 * You must write an algorithm with O(log n) runtime complexity.
 *
 * Example:
 *
 *
 * Input: nums = [-1,0,3,5,9,12], target = 9
 * Output: 4
 * Explanation: 9 exists in nums and its index is 4
 *
 * Problem: https://leetcode.com/problems/binary-search/
 */

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;

        let (mut lo, mut hi) = (0, nums.len());

        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            match nums[mid as usize].cmp(&target) {
                Ordering::Less => {
                    lo = mid + 1;
                }
                Ordering::Greater => {
                    hi = mid;
                }
                Ordering::Equal => {
                    return mid as i32;
                }
            }
        }

        -1
    }
}

#[test]
fn test() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 9;
    assert_eq!(Solution::search(nums, target), 4);

    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 2;
    assert_eq!(Solution::search(nums, target), -1);
}
