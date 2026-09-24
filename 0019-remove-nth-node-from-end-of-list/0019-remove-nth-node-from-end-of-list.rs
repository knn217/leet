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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut list_len = 0;
        {
            let mut current = head.as_deref();
            while let Some(node) = current {
                // println!("current node: {:?}", node);
                current = node.next.as_deref(); // converts Option<Box<ListNode>> to Option<&ListNode>
                list_len += 1;
            }
            // println!("list_len {}", list_len);
        }
        // Edge case
        if list_len == 1 && n == 1 { return None; }
        if list_len > 1 && n == list_len { return head.unwrap().next; }
        // Normal case
        let mut head = head;
        let mut current = &mut head;
        let mut next = None;
        let mut count = 0;
        while current.is_some() {
            if count >= (list_len - 1 - n) {
                next = current.as_mut().unwrap().next.clone();
                // println!("next: {:?}", next);
                if let Some(node) = next {
                    next = node.next.clone();
                }
                break;
            }
            // println!("{}", current.as_mut().unwrap().val);
            current = &mut current.as_mut().unwrap().next;
            // println!("current: {:?}", current);
            count += 1;
        }
        current.as_mut().unwrap().next = next;
        head
    }
}