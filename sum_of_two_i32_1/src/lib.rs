#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use crate::solution::*;

    #[test]
    fn it_works() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn other_test() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
}

mod solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..(nums.len() - 1) {
            for j in (i + 1)..(nums.len()) {
                if nums[j] == (target - nums[i]) {
                    return vec![i as i32, j as i32];
                } else {
                    continue;
                }
            }
        }
        vec![0]
    }
}
