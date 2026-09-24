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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut list_len = 0;
        {
            let mut current = head.as_deref();
            while let Some(node) = current {
                current = node.next.as_deref();
                list_len += 1;
            }
        }
        let mut k = k;
        if list_len == 0 { return head; }
        k %= list_len;
        // println!("list_len: {:?}", list_len);
        //  Edge case
        if list_len <= 1 || k == 0 || k == list_len || list_len == 0 { return head; }
        // Normal cases
        let mut head = head;
        let mut head_new = Some(Box::new(ListNode::new(0)));
        let mut current = &mut head;
        let mut count = 0;
        while let Some(node) = current {
            count += 1;
            if count >= (list_len - k) {
                head_new = node.next.take();
                // println!("head_new: {:?}", head_new);
                // println!("head: {:?}", head);
                break;
            }
            current = &mut node.next;
            // println!("current: {:?}", current);
        }
        let mut current = &mut head_new;
        while let Some(node) = current {
            if node.next == None {
                node.next = head.take();
                break;
            }
            current = &mut node.next;
            // println!("current: {:?}", current);
        }
        head_new
    }
}