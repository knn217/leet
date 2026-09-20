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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Edge cases
        if list1 == None { return list2; }
        if list2 == None { return list1; }
        // Normal cases
        let mut list1 = list1;
        let mut list2 = list2;
        let mut list_start;
        let mut list_other;
        if list1.as_ref().unwrap().val > list2.as_ref().unwrap().val {
            list_start = list2.clone();
            list_other = list1;
        } else {
            list_start = list1.clone();
            list_other = list2;
        }
        // Current is guarenteed to be the smallest starting value
        let mut current = list_start.as_mut();
        while let Some(node) = current.take() {
            if list_other == None {
                println!("Other is none, all nodes are in main list");
                break;
            }
            if node.next == None {
                println!("Main's next is none, all remaining nodes are in other list, moving other into main's next");
                node.next = list_other.take();
                break;
            }
            let next_val = node.next.as_ref().unwrap().val;
            let other_val = list_other.as_ref().unwrap().val;
            if next_val >= other_val {
                println!("other's current value {} <= next value {}, swapping other and next", other_val, next_val);
                println!("node.next: {:?}", node.next);
                println!("list_other: {:?}", list_other);
                let tmp = node.next.take();
                println!("tmp: {:?}", tmp);
                node.next = list_other.take();
                list_other = tmp;
            }
            println!("{}", node.val);
            current = node.next.as_mut();
        }
        println!("{:?}", list_start);
        list_start
    }
}