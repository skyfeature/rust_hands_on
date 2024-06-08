struct Solution {

}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums: Vec<_> = nums.iter().enumerate().map(|(i, x)| (i, x)).collect();

        nums.sort_by_key(|pair| pair.1);

        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let pair_sum = nums[left].1 + nums[right].1;
            if pair_sum == target {
                return vec![nums[left].0 as i32, nums[right].0 as i32];
            }
            else if pair_sum < target {
                left += 1;
            }
            else {
                right -= 1;
            }
        }

        vec![0, 0]


    }
}