/**
 * [9] Palindrome Number
 *
 * Given an integer x, return true if x is palindrome integer.
 *
 * An integer is a palindrome when it reads the same backward as forward.
 *
 * Example:
 *
 *
 * Input: x = 121
 * Output: true
 * Explanation: 121 reads as 121 from left to right and from right to left.
 *
 * Problem: https://leetcode.com/problems/palindrome-number/
 */

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string() == x.to_string().chars().rev().collect::<String>()
    }

    pub fn is_palindrome_without_conversion(x: i32) -> bool {
        if x < 0 {
            false
        } else {
            let (mut rev, mut num) = (0_i32, x);

            while num > 0 {
                rev = rev * 10 + num % 10;
                num /= 10;
            }

            rev == x
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(783387), true);
    assert_eq!(Solution::is_palindrome(123), false);

    assert_eq!(Solution::is_palindrome_without_conversion(121), true);
    assert_eq!(Solution::is_palindrome_without_conversion(783387), true);
    assert_eq!(Solution::is_palindrome_without_conversion(123), false);
}
