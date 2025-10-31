/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
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
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {next: head, val: 0});
        let mut prev = dummy.as_mut();

        while prev.next.is_some() && prev.next.as_ref().unwrap().next.is_some() {
            let mut node1 = prev.next.take().unwrap();
            let mut node2 = node1.next.take().unwrap();

            node1.next = node2.next.take();
            node2.next = Some(node1);
            prev.next = Some(node2);

            prev = prev.next.as_mut().unwrap().next.as_mut().unwrap();
        }

        dummy.next
    }
}
// @lc code=end

