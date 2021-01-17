#[cfg(test)]
mod tests {
    use crate::kids_with_candies;

    #[test]
    fn it_works() {
        assert_eq!(
            kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );
    }

    #[test]
    fn another_works() {}
}
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut result = Vec::new();
    let max = candies.iter().max().unwrap();
    candies.iter().for_each(|x| {
        if *x + extra_candies >= *max {
            result.push(true)
        } else {
            result.push(false)
        };
    });
    result
}
