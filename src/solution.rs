impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut head = None;
        let mut tail = &mut head;
        let mut carry = false;
        loop {
            if l1 == None && l2 == None {
                break;
            }
            let n1 = if let Some(l) = l1 {
                let r = l.val;
                l1 = l.next;
                r
            } else {
                0
            };
            let n2 = if let Some(l) = l2 {
                let r = l.val;
                l2 = l.next;
                r
            } else {
                0
            };
            let n = n1 + n2 + if carry { 1 } else { 0 };
            carry = n > 9;
            let digit = n % 10;
            *tail = Some(Box::new(ListNode::new(digit)));
            if let Some(current) = tail {
                tail = &mut current.next;
            }
        }
        if carry {
            *tail = Some(Box::new(ListNode::new(1)));
        }
        head
    }
}

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
}

pub struct Solution;
