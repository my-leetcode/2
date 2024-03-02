use std::io::{self, Write};

use crate::solution::{ListNode, Solution};

mod solution;

fn to_list(str: &str) -> ListNode {
    let mut head = None;
    let mut tail = &mut head;
    for digit in str.chars().rev() {
        *tail = Some(Box::new(ListNode {
            val: String::from(digit).parse().unwrap(),
            next: None,
        }));
        if let Some(current) = tail {
            tail = &mut current.next;
        }
    }
    if let Some(result) = head {
        *result
    } else {
        ListNode { val: 0, next: None }
    }
}

fn main() {
    print!("Enter number 1 > ");
    let mut line = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).unwrap();
    let num1 = to_list(line.trim());

    print!("Enter number 2 > ");
    let mut line = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).unwrap();
    let num2 = to_list(line.trim());

    println!(
        "{:#?}",
        Solution::add_two_numbers(Some(Box::new(num1)), Some(Box::new(num2)))
    )
}
