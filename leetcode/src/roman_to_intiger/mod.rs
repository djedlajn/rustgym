struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.as_bytes().iter().fold([0; 4], |[m, c, x, i], b| {
            match *b {
                b'M' => [m + 1000 - c, 0, x, i],
                b'D' => [m + 500 - c, 0, x, i],
                b'C' => [m, c + 100 - x, 0, i],
                b'L' => [m, c + 50 - x, 0, i],
                b'X' => [m, c, x + 10 - i, 0],
                b'V' => [m, c, x + 5 - i, 0],
                _ => [m, c, x, i + 1],
            }
        }).into_iter().sum::<i32>()
    }
}
#[test]
fn test() {
    assert_eq!(Solution::roman_to_int("III".to_owned()), 3);
}