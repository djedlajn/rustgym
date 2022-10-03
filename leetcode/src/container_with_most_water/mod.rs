// You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

// Find two lines that together with the x-axis form a container, such that the container contains the most water.

// Return the maximum amount of water a container can store.

// Notice that you may not slant the container.

// Problem: https://leetcode.com/problems/container-with-most-water/

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut res, mut left, mut right) = (0, 0, height.len() - 1);
        
        while left < right {
            let x = (right - left) as i32;
            let mut y = 0;
            
            if height[left] < height[right] {
                y = height[left];
                left += 1;
            } else {
                y = height[right];
                right -= 1;
            }

            res = res.max(x * y);
        }
        res
    }
}


#[test]
fn test() {
    assert_eq!(Solution::max_area([1,1].to_vec()), 1);
}