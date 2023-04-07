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

    let node = add_two_numbers(n1,n2);

    println!("{:?}",node);

}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut op1: Vec<i32>  = Vec::new();
    let mut node = l1.as_ref();

    while let Some(n) = node {
        op1.push(n.val);
        node = n.next.as_ref();
    }

    let mut op2: Vec<i32>  = Vec::new();
    let mut node2 = l2.as_ref();

    while let Some(n) = node2 {
        op2.push(n.val);
        node2 = n.next.as_ref();
    }

    op1.reverse();
    op2.reverse();

    let mut sum = add_vector_of_digits(op1,op2);

    sum.reverse();
    
    vec_of_digits_to_linkedlist(sum)
    
}

pub fn add_vector_of_digits(left_operand: Vec<i32>, right_operand: Vec<i32>) -> Vec<i32>{
    let (mut smaller_operand, mut larger_operand) = match left_operand.len() < right_operand.len() {
        true => (left_operand, right_operand),
        false => (right_operand, left_operand),
    };

    smaller_operand.reverse();
    larger_operand.reverse();

    let mut result: Vec<i32> = Vec::new();
    let mut carry: i32 = 0;

    for (index,value) in larger_operand.iter().enumerate(){
        let right = match smaller_operand.get(index){
            Some(j) => *j,
            None => 0,
        };
        let column_sum = value + right + carry;

        if column_sum > 9 {
            carry = 1;
            result.push(column_sum % 10);
        } else {
            carry = 0;
            result.push(column_sum);
        }
    }

    if carry == 1 { result.push(1);}

    result.reverse();
    result
}

pub fn integer_to_linked_list(num: i128) -> Option<Box<ListNode>>{
    let digits: Vec<i32> = num_to_digits(num);
    vec_of_digits_to_linkedlist(digits)
}

fn vec_of_digits_to_linkedlist(digits: Vec<i32>) -> Option<Box<ListNode>> {
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
