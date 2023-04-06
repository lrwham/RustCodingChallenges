// leetcode #2
// You are given two non-empty linked lists representing two
// non-negative integers. The digits are stored in reverse order,
// and each of their nodes contains a single digit.
// 
// Add the two numbers and return the sum as a linked list.
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.


pub fn test(n1: i128, n2: i128){
    println!("{}",n1);
    println!("{}",n2);
    let n1 = integer_to_linked_list(n1);
    let n2 = integer_to_linked_list(n2);

    println!("{:?}",n1);
    println!("{:?}",n2);

    let mut node = add_two_numbers(n1,n2);

    println!("{:?}",node);

}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut op1: i128 = 0;
    let mut op1_digit_count: u32 = 0;
    let mut node = l1.as_ref();

    while let Some(n) = node {
        op1 += n.val as i128 * i128::pow(10, op1_digit_count);
        op1_digit_count += 1;
        node = n.next.as_ref();
    }

    let mut op2: i128 = 0;
    let mut op2_digit_count: u32 = 0;
    let mut node2 = l2.as_ref();

    while let Some(n) = node2 {
        op2 += n.val as i128 * i128::pow(10, op2_digit_count);
        op2_digit_count += 1;
        node2 = n.next.as_ref();
    }


    let sum: i128 = op1 + op2;

    let result =  integer_to_linked_list(sum);

    result
    
}

pub fn integer_to_linked_list(num: i128) -> Option<Box<ListNode>>{
    let mut digits: Vec<i32> = num_to_digits(num);
    digits_to_linkedlist(digits)
}

fn digits_to_linkedlist(digits: Vec<i32>) -> Option<Box<ListNode>> {
    // Special case for empty input vector
    if digits.is_empty() {
        return None;
    }

    // Create the first node of the linked list
    let mut head = Box::new(ListNode::new(digits[0] as i32));
    let mut tail = &mut head;

    // Iterate over the remaining digits and add them to the linked list
    for digit in digits.into_iter().skip(1) {
        let node = Box::new(ListNode::new(digit as i32));
        tail.next = Some(node);
        tail = tail.next.as_mut().unwrap();
    }

    Some(head)
}

fn num_to_digits(num: i128) -> Vec<i32> {
    let mut digits = vec![];
    let mut num = num.abs(); // Take the absolute value to handle negative numbers

    // Special case for 0
    if num == 0 {
        return vec![0];
    }

    while num > 0 {
        let digit = num % 10;
        digits.push(digit as i32);
        num /= 10;
    }

    digits
}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
        next: None,
        val
        }
    }
}
