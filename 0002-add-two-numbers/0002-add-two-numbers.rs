// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Handle edge cases
        if l1 == None { return l2; }
        if l2 == None { return l1; }
        // Handle normal case
        let mut l3 = Some(Box::new(ListNode::new(0)));
        let mut l1 = l1.as_deref();
        let mut l2 = l2.as_deref();
        let mut l3_current = l3.as_deref_mut();
        let mut carryover = 0;
        while l1.is_some() || l2.is_some() || carryover != 0 {
            let mut sum = carryover;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next.as_deref();
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next.as_deref();
            }
            carryover = sum / 10;
            println!("{}, {}", sum, carryover);
            if let Some(node) = l3_current {
                node.next = Some(Box::new(ListNode::new(sum % 10)));
                l3_current = node.next.as_deref_mut();
            }
        }
        l3.unwrap().next
    }
}