#[cfg(test)]
mod tests {
    use crate::reverse_left_words;

    #[test]
    fn it_works() {
        assert_eq!(
            reverse_left_words("abcdefg".to_string(), 2),
            "cdefgab".to_string()
        )
    }

    #[test]
    fn other_test(){
        assert_eq!(
            reverse_left_words("lrloseumgh".to_string(), 6),
            "umghlrlose"
        )
    }
}
pub fn reverse_left_words(s: String, n: i32) -> String {
    let n = n as usize;
    format!("{}{}", s.get(n..(s.len())).unwrap(),s.get(0..n).unwrap())

}
