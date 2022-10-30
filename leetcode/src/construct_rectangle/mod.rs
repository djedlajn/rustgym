// A web developer needs to know how to design a web page's size. So, given a specific rectangular web page’s area, your job by now is to design a rectangular web page, whose length L and width W satisfy the following requirements:

// The area of the rectangular web page you designed must equal to the given target area.
// The width W should not be larger than the length L, which means L >= W.
// The difference between length L and width W should be as small as possible.
// Return an array [L, W] where L and W are the length and width of the web page you designed in sequence.

struct Solution;

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut x = (area as f32).sqrt() as i32;
        loop {
            match area % x {
                0 => break vec![area / x, x],
                _ => x -= 1,
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::construct_rectangle(37), [37, 1]);
}