// Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.

// The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the frequency of at least one of the chosen numbers is different.

// The test cases are generated such that the number of unique combinations that sum up to target is less than 150 combinations for the given input.

// Input: candidates = [2,3,6,7], target = 7
// Output: [[2,2,3],[7]]
// Explanation:
// 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
// 7 is a candidate, and 7 = 7.
// These are the only two combinations

struct Solution;

impl Solution {
    fn combi(res: &mut Vec<Vec<i32>>, buffer: &mut Vec<i32>, candidates: &[i32], target: &i32) {
        buffer.push(candidates[0]);

        if buffer.iter().sum::<i32>()==*target {
            res.push(buffer.clone());
        } else if buffer.iter().sum::<i32>()<*target {
            for i in 0..candidates.len() {
                Solution::combi(res, buffer, &candidates[i..], target);
            }
        }

        buffer.pop();
    }
    
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut buffer: Vec<i32>;
        for i in 0..candidates.len() {
            buffer = vec![];
            Solution::combi(&mut res, &mut buffer, &candidates[i..], &target);
        }
        res
    }
}