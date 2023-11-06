// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     pub fn new(val: i32) -> ListNode {
//         ListNode { val, next: None }
//     }
// }

// // 为链表节点添加一个构建链表的辅助函数
// impl ListNode {
//     pub fn from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
//         let mut head = None;
//         let mut current = &mut head;

//         for &val in values.iter() {
//             let new_node = ListNode::new(val);
//             *current = Some(Box::new(new_node));
//             current = &mut current.as_mut().unwrap().next;
//         }

//         head
//     }
// }

// pub fn add_two_numbers(
//     l1: Option<Box<ListNode>>,
//     l2: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
//     let (mut l1, mut l2) = (l1, l2);
//     let mut result = None;
//     let mut carry = 0;

//     let mut current = &mut result;

//     while l1.is_some() || l2.is_some() {
//         let mut sum = carry;

//         if let Some(node) = l1 {
//             sum += node.val;
//             l1 = node.next;
//         }

//         if let Some(node) = l2 {
//             sum += node.val;
//             l2 = node.next;
//         }

//         carry = sum / 10;
//         *current = Some(Box::new(ListNode::new(sum % 10)));
//         current = &mut current.as_mut().unwrap().next;
//     }

//     if carry > 0 {
//         *current = Some(Box::new(ListNode::new(carry)));
//     }

//     result
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn add_two_numbers() {
//         // 创建链表1: 2 -> 4 -> 3
//         let l1 = Some(Box::new(ListNode::new(2)));
//         let l1 = Some(Box::new(ListNode { val: 4, next: l1 }));
//         let l1 = Some(Box::new(ListNode { val: 3, next: l1 }));

//         // 创建链表2: 5 -> 6 -> 4
//         let l2 = Some(Box::new(ListNode::new(5)));
//         let l2 = Some(Box::new(ListNode { val: 6, next: l2 }));
//         let l2 = Some(Box::new(ListNode { val: 4, next: l2 }));

//         // 预期结果链表: 7 -> 0 -> 8
//         let expected_result = Some(Box::new(ListNode::new(7)));
//         let expected_result = Some(Box::new(ListNode {
//             val: 5,
//             next: expected_result,
//         }));
//         let expected_result = Some(Box::new(ListNode {
//             val: 8,
//             next: expected_result,
//         }));

//         assert_eq!(add_two_numbers(l1, l2), expected_result);
//     }
// }
