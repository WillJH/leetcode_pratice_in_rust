#[cfg(test)]
mod tests {
    use crate::min_partitions;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn check_min_partitions() {
        assert_eq!(min_partitions("32".to_string()), 3);
    }
    #[test]
    fn check_min_partitions_2() {
        assert_eq!(min_partitions("82734".to_string()), 8);
    }
    #[test]
    fn check_min_partitions_3() {
        assert_eq!(min_partitions("27346209830709182346".to_string()), 9);
    }
}
pub fn min_partitions(n: String) -> i32 {
    n.chars().max().unwrap().to_string().parse().unwrap()
}
