#[cfg(test)]
mod tests {
    use crate::running_sum;

    #[test]
    fn it_works() {
        assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    }

    #[test]
    fn other_test() {
        assert_eq!(running_sum(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17])
    }
}
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut temp = 0_i32;
    let result = nums
        .iter()
        .map(|&val| {
            temp += val;
            temp
        })
        .collect();
    result
}
