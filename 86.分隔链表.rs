/*
 * @lc app=leetcode.cn id=86 lang=rust
 *
 * [86] 分隔链表
 */

// @lc code=start
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
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut dummy1 = Box::new(ListNode::new(0));
        let mut l1 = dummy1.as_mut();
        let mut dummy2 = Box::new(ListNode::new(0));
        let mut l2 = dummy2.as_mut();

        while let Some(mut node) = head {
            head = node.next.take();

            if node.val < x {
                l1.next = Some(node);
                l1 = l1.next.as_mut().unwrap();
            } else {
                l2.next = Some(node);
                l2 = l2.next.as_mut().unwrap();
            }
        }

        l1.next = dummy2.next;

        dummy1.next

    }
}
// @lc code=end

