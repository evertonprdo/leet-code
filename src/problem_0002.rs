// 2. Add Two Numbers: https://leetcode.com/problems/add-two-numbers

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn push(&mut self, val: i32) {
        let mut new_node = Box::new(ListNode::new(val));
        self.next = match self.next.take() {
            Some(node) => {
                new_node.next = Some(node);
                Some(new_node)
            }
            None => Some(new_node),
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.next.take() {
            Some(node) => {
                self.next = node.next;
                Some(node.val)
            }
            None => None,
        }
    }
}

pub struct Solution {}
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut root = ListNode::new(0);
        let mut tail = &mut root.next;

        loop {
            let x = Self::pop(&mut l1);
            let y = Self::pop(&mut l2);

            if x.is_none() && y.is_none() && carry == 0 {
                break;
            }

            let r = carry + x.unwrap_or(0) + y.unwrap_or(0);
            carry = r / 10;

            *tail = Some(Box::new(ListNode::new(r % 10)));
            tail = &mut tail.as_mut().unwrap().next;
        }

        root.next
    }

    fn pop(node: &mut Option<Box<ListNode>>) -> Option<i32> {
        match node.take() {
            Some(n) => {
                let val = n.val;
                *node = n.next;

                Some(val)
            }
            None => None,
        }
    }
}

pub struct IterativeSolution {}
impl IterativeSolution {
    // https://leetcode.com/problems/add-two-numbers/solutions/5934960/rust-iterative-approach-0-ms/
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut tail = &mut head;

        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let sum = l1.as_ref().map_or(0, |x| x.val) + l2.as_ref().map_or(0, |x| x.val) + carry;
            carry = sum / 10;

            let sum_node = tail.insert(Box::new(ListNode::new(sum % 10)));
            tail = &mut sum_node.next;

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        head
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let l1 = [2, 4, 3];
        let l2 = [5, 6, 4];
        let output = [7, 0, 8];

        let mut root_l1 = ListNode::new(0);
        let mut root_l2 = ListNode::new(0);
        for &i in l1.iter().rev() {
            root_l1.push(i);
        }
        for &i in l2.iter().rev() {
            root_l2.push(i);
        }

        let mut root = ListNode::new(0);
        root.next = Solution::add_two_numbers(root_l1.next, root_l2.next);

        for i in output {
            assert_eq!(root.pop(), Some(i))
        }

        let mut root_l1 = ListNode::new(0);
        let mut root_l2 = ListNode::new(0);
        for &i in l1.iter().rev() {
            root_l1.push(i);
        }
        for &i in l2.iter().rev() {
            root_l2.push(i);
        }

        let mut root = ListNode::new(0);
        root.next = IterativeSolution::add_two_numbers(root_l1.next, root_l2.next);

        for i in output {
            assert_eq!(root.pop(), Some(i))
        }
    }

    #[test]
    fn example_02() {
        let l1 = [0];
        let l2 = [0];
        let output = [0];

        let mut root_l1 = ListNode::new(0);
        let mut root_l2 = ListNode::new(0);
        for &i in l1.iter().rev() {
            root_l1.push(i);
        }
        for &i in l2.iter().rev() {
            root_l2.push(i);
        }

        let mut root = ListNode::new(0);
        root.next = Solution::add_two_numbers(root_l1.next, root_l2.next);

        for i in output {
            assert_eq!(root.pop(), Some(i))
        }

        let mut root_l1 = ListNode::new(0);
        let mut root_l2 = ListNode::new(0);
        for &i in l1.iter().rev() {
            root_l1.push(i);
        }
        for &i in l2.iter().rev() {
            root_l2.push(i);
        }

        let mut root = ListNode::new(0);
        root.next = IterativeSolution::add_two_numbers(root_l1.next, root_l2.next);

        for i in output {
            assert_eq!(root.pop(), Some(i))
        }
    }

    #[test]
    fn example_03() {
        let l1 = [9, 9, 9, 9, 9, 9, 9];
        let l2 = [9, 9, 9, 9];
        let output = [8, 9, 9, 9, 0, 0, 0, 1];

        let mut root_l1 = ListNode::new(0);
        let mut root_l2 = ListNode::new(0);
        for &i in l1.iter().rev() {
            root_l1.push(i);
        }
        for &i in l2.iter().rev() {
            root_l2.push(i);
        }

        let mut root = ListNode::new(0);
        root.next = Solution::add_two_numbers(root_l1.next, root_l2.next);

        for i in output {
            assert_eq!(root.pop(), Some(i))
        }

        let mut root_l1 = ListNode::new(0);
        let mut root_l2 = ListNode::new(0);
        for &i in l1.iter().rev() {
            root_l1.push(i);
        }
        for &i in l2.iter().rev() {
            root_l2.push(i);
        }

        let mut root = ListNode::new(0);
        root.next = IterativeSolution::add_two_numbers(root_l1.next, root_l2.next);

        for i in output {
            assert_eq!(root.pop(), Some(i))
        }
    }
}
