use std::collections::HashSet;
#[cfg(test)]
mod tests {
    use crate::num_jewels_in_stones;

    #[test]
    fn it_works() {
        assert_eq!(
            num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
            3
        );
    }
    #[test]
    fn another_test() {
        assert_eq!(num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
    }
}
pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut count =0_i32;
    let jewels_set:HashSet<char> = jewels.chars().collect();

    stones.chars().for_each(|c| {
        if let Some(_) = jewels_set.get(&c){
            count+=1;
        }
    });
    
    count
}
