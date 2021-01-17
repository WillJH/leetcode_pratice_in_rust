use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::num_identical_pairs;

    #[test]
    fn it_works() {
        assert_eq!(num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    }

    #[test]
    fn another_test() {
        assert_eq!(num_identical_pairs(vec![1, 1, 1, 1]), 6)
    }
}

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, Vec<usize>> = HashMap::new();

    for (index, &val) in nums.iter().enumerate() {
        match map.get(&val) {
            Some(_) => map.get_mut(&val).unwrap().push(index),
            None => {
                map.insert(val, vec![index]);
            }
        }
    }

    let mut total_result_count = 0usize;
    
    for value in map.values() {
        total_result_count += ((value.len()-1)*value.len())/2;
    }

    total_result_count as i32
}
