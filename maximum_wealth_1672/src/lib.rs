#[cfg(test)]
mod tests {
    use crate::maximum_wealth;

    #[test]
    fn it_works() {
        assert_eq!(maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]), 6);
    }

    #[test]
    fn anther_test() {
        assert_eq!(maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![5, 3]]), 10);
    }

    #[test]
    fn another_test() {
        assert_eq!(
            maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]),
            17
        );
    }
}

pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.iter().map(|i| i.iter().sum()).max().unwrap()
}
