// 203. Remove Linked List Elements: https://leetcode.com/problems/remove-linked-list-elements

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
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
    // https://leetcode.com/problems/remove-linked-list-elements/solutions/2327783/most-intuitive-rust-for-me/
    // I reached the same idea, but sometimes it's just skill issues.
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut curr = &mut head;
        loop {
            match curr {
                None => break,
                Some(node) if node.val == val => {
                    *curr = node.next.take();
                }
                Some(node) => curr = &mut node.next,
            }
        }
        head
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let input = [1, 2, 6, 3, 4, 5, 6];
        let val = 6;
        let output = [1, 2, 3, 4, 5];

        let mut root = ListNode::new(0);
        for &i in input.iter().rev() {
            root.push(i);
        }

        root.next = Solution::remove_elements(root.next, val);
        for i in output {
            assert_eq!(root.pop(), Some(i))
        }
        assert_eq!(root.pop(), None);
    }

    #[test]
    fn example_02() {
        let input = [];
        let val = 1;
        let output = [];

        let mut root = ListNode::new(0);
        for &i in input.iter().rev() {
            root.push(i);
        }

        root.next = Solution::remove_elements(root.next, val);
        for i in output {
            assert_eq!(root.pop(), Some(i))
        }
        assert_eq!(root.pop(), None);
    }

    #[test]
    fn example_03() {
        let input = [7, 7, 7, 7];
        let val = 7;
        let output = [];

        let mut root = ListNode::new(0);
        for &i in input.iter().rev() {
            root.push(i);
        }

        root.next = Solution::remove_elements(root.next, val);
        for i in output {
            assert_eq!(root.pop(), Some(i))
        }
        assert_eq!(root.pop(), None);
    }
}
