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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Handle edge cases
        if l1 == None { return l2; }
        if l2 == None { return l1; }
        // Handle normal case
        let mut l1_ = l1.as_deref_mut();
        let mut l2_ = l2.as_deref_mut();
        let mut carryover = 0;
        while l1_.is_some() || l2_.is_some() || carryover != 0 {
            let mut sum = carryover;
            if let Some(node) = l1_ {
                sum += node.val;
                l1_ = node.next.as_deref_mut();
            }
            if let Some(node) = l2_ {
                sum += node.val;
                carryover = sum / 10;
                node.val = sum % 10;
                if node.next == None && l1_ != None {
                    node.next = Some(Box::new(l1_.unwrap().clone()));
                    l1_ = None;
                }
                if node.next == None && carryover != 0 {
                    node.next = Some(Box::new(ListNode::new(0)));
                }
                l2_ = node.next.as_deref_mut();
            }
            // println!("{}, {}", sum, carryover);
        }
        l2
    }
}