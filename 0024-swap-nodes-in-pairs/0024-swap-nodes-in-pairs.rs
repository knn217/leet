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
pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut current = &mut dummy;

        // Ensure both the first and second nodes exist before swapping
        while current.next.is_some() && current.next.as_ref().unwrap().next.is_some() {
            // NOTE:
            // - Use take() whenever we want left side to right side's next nodes -> Right side becomes single node
            // - Use Some() whenever we want right side to becomes left side's next
            // Take ownership of node1 and its chain
            let mut node1 = current.next.take().unwrap();
            // Take ownership of node2 and the rest of the list
            let mut node2 = node1.next.take().unwrap();

            // Re-link: node1's next points to node2's original next
            node1.next = node2.next.take();
            // node2's next points to node1
            node2.next = Some(node1);
            // Re-attach swapped sub-list (starting with node2) to current
            current.next = Some(node2);

            // Advance current pointer two nodes ahead (to node1)
            current = current.next.as_mut().unwrap().next.as_mut().unwrap();
        }

        dummy.next
    }
}