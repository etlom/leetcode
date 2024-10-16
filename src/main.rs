

fn main() {
    let nums = vec![7,8,3,4,15,13,4,1];
    println!("res = {}", Solution::minimum_average(nums));
}

struct Solution;

impl Solution {
    pub fn minimum_average(nums: Vec<i32>) -> f64 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut res = f64::INFINITY;
        for i in 0..n/2 {
            let average = ((nums[i] + nums[n - i - 1]) as f64) / 2.0;
            res = res.min(average);
        }
        res
    }
}