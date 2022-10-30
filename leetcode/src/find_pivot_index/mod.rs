/*
Given an array of integers nums, calculate the pivot index of this array.

The pivot index is the index where the sum of all the numbers strictly to the left of the index is equal to the sum of all the numbers strictly to the index's right.

If the index is on the left edge of the array, then the left sum is 0 because there are no elements to the left. This also applies to the right edge of the array.

Return the leftmost pivot index. If no such index exists, return -1.

Problem: https://leetcode.com/problems/find-pivot-index/
 */
 

struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum:i32 = nums.iter().sum();
        let mut left = 0;
        let mut right = sum;
        for i in 0..nums.len() {
            right = sum - nums[i] - left;
            if right == left {
                return i as i32;
            }
            left += nums[i];
        }
        -1
    }
}