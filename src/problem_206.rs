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
}

pub struct Solution {}

impl Solution {
    pub fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &i in vec.iter().rev() {
            let mut node = Box::new(ListNode::new(i));
            node.next = head;
            head = Some(node);
        }
        head
    }

    pub fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut head = head;
        let mut vec: Vec<i32> = Vec::new();
        while let Some(node) = head {
            vec.push(node.val);
            head = node.next;
        }
        vec
    }

    pub fn reverse_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = None;
        let mut current = head;
        while let Some(mut node) = current {
            let next_node = node.next;
            node.next = new_head;
            new_head = Some(node);
            current = next_node;
        }
        new_head
    }
}
