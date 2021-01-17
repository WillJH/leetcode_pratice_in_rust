#[cfg(test)]
mod tests {
    use crate::ListNode;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn new_a_list_node_from_vec() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7];
        print!("{:#?}", ListNode::new_from_vec(v));
    }
}

mod solution {
    use crate::ListNode;
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut sum = l1.unwrap().summary_of_sub_node_values() +l2.unwrap().summary_of_sub_node_values();

        let mut result = Some(Box::new(ListNode::new(sum % 10)));

        //TODO calculate the separate value and create a new node
    }

}
// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn new_from_vec(vals: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>>;
        if vals.len() > 0 {
            head = Some(Box::new(ListNode::new(vals[0])));
        } else {
            head = None;
        }

        if head != None {
            for i in 1..vals.len() {
                head.as_mut().unwrap().append(vals[i]);
            }
        }
        head
    }

    fn append(&mut self, val: i32) {
        match self.next {
            Some(ref mut n) => n.append(val),
            None => self.next = Some(Box::new(Self::new(val))),
        }
    }

    fn accumulate(&self, mut base: i32) -> i32 {
        match self.next{
            Some(ref n) => base += n.accumulate(base * 10),
            None => base = self.val,
        }
        base
    }
    
    fn summary_of_sub_node_values(&self)->i32{
        self.accumulate(0)
    }
}
