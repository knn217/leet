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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = &mut head;
        while current.is_some() {
            // Borrow node for THIS iteration only
            let node = current.as_mut().unwrap();
            // Check for final
            if node.next == None { break; }
            let next_val = node.next.as_ref().unwrap().val;
            if next_val == node.val {
                // If next is repeated, next take next's next, don't progress
                node.next = node.next.as_mut().unwrap().next.take();
            } else {
                // If not repeated, progress
                current = &mut current.as_mut().unwrap().next;
            }
            // =====
            // println!("current: {:?}", current);
        }
        // println!("head: {:?}", head);
        head
    }
}