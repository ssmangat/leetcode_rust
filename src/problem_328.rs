//Defining Struct for singly-linked list
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
        for &num in vec.iter().rev() {
            let mut node = Box::new(ListNode::new(num));
            node.next = head;
            head = Some(node);
        }
        head
    }

    pub fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        while let Some(node) = head {
            vec.push(node.val);
            head = node.next;
        }
        vec
    }

    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let (mut odd, mut even) = (None, None);
        {
            let (mut _odd_head, mut even_head) = (&mut odd, &mut even);
            let mut flag: i32 = 0;
            while let Some(mut node) = head {
                head = node.next;
                node.next = None;
                flag = 1 - flag;
                if flag == 1 {
                    _odd_head = &mut _odd_head.insert(node).next;
                } else {
                    even_head = &mut even_head.insert(node).next;
                }
            }
            if let Some(node) = even {
                _odd_head = &mut _odd_head.insert(node).next;
            }
        }
        odd
    }
}
