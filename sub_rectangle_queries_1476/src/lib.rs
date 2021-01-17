#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use crate::SubrectangleQueries;

    #[test]
    fn it_works() {
        // assert_eq!(2 + 2, 4);
        let mut s = SubrectangleQueries::new(vec![
            vec![1, 2, 1],
            vec![4, 3, 4],
            vec![3, 2, 1],
            vec![1, 1, 1],
        ]);

        assert_eq!(s.get_value(0, 2), 1);

        s.update_subrectangle(0, 0, 3, 2, 5);

        assert_eq!(
            s,
            SubrectangleQueries::new(vec![
                vec![5, 5, 5],
                vec![5, 5, 5],
                vec![5, 5, 5],
                vec![5, 5, 5],
            ])
        );

        assert_eq!(s.get_value(0, 2), 5);
        assert_eq!(s.get_value(3, 1), 5);

        s.update_subrectangle(3, 0, 3, 2, 10);

        assert_eq!(
            s,
            SubrectangleQueries::new(vec![
                vec![5, 5, 5],
                vec![5, 5, 5],
                vec![5, 5, 5],
                vec![10, 10, 10],
            ])
        );
    }
}
#[derive(Debug, PartialEq, Eq)]
struct SubrectangleQueries {
    rectangle: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self { rectangle }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for i in row1..(row2 + 1) {
            for j in col1..(col2 + 1) {
                self.rectangle[i as usize][j as usize] = new_value;
            }
        }
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        self.rectangle[row as usize][col as usize]
    }
}
