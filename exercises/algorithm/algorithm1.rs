/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
        where T: std::cmp::Ord


    {
        //TODO
        /*// 取出元素
        let mut v_a = Vec::new();
        if let Some(node) = list_a.start {
            v_a.push(unsafe { node.as_ptr()});
        }
        /*match list_a.end {
            None =>  // 只是结束而已，不该返回任何值，此时应该使用 if/while let
        }*/
        while let Some(node) = list_a.end {
            v_a.push(unsafe { node.as_ptr()});
        }

        let mut v_b = Vec::new();
        if let Some(node) = list_b.start {
            v_b.push(unsafe { node.as_ptr() });
        }
        while let Some(node) = list_b.end {
            v_b.push(unsafe { node.as_ptr() });
        }
        v_a.append(&mut v_b);
        v_a.sort();
        let mut ll = LinkedList::<T>::default();
        for i in v_a {
            ll.add(i);
        }
        ll*/

        let len = list_a.length + list_b.length;
        let mut node_a: Option<NonNull<Node<T>>> = list_a.start;
        let mut node_b: Option<NonNull<Node<T>>> = list_b.start;

        if node_b == None { node_a = node_b; }

        let mut p: Option<NonNull<Node<T>>> =
            unsafe {
                if (*node_a.unwrap().as_ptr()).val < (*node_b.unwrap().as_ptr()).val {
                    let ret = std::mem::take(&mut node_a);
                    node_a = (*ret.unwrap().as_ptr()).next;
                    ret
                } else {
                    let ret = std::mem::take(&mut node_b);
                    node_b = (*ret.unwrap().as_ptr()).next;
                    ret
                }
            };

        assert_ne!(node_b, None);
        assert_ne!(node_a, None);

        let start = p; //NonNullÊµÏÖÁËCopy Trait
        unsafe {
            while node_a != None && node_b != None {
                if (*node_a.expect("it should be not none").as_ptr()).val < (*node_b.expect("it should be not none").as_ptr()).val {
                    (*p.expect("it should be not none").as_ptr()).next = node_a;
                    p = node_a;
                    node_a = (*node_a.expect("it should be not none").as_ptr()).next;
                } else {
                    (*p.expect("it should be not none").as_ptr()).next = node_b;
                    p = node_b;
                    node_b = (*node_b.expect("it should be not none").as_ptr()).next;
                };
            }
        }

        if node_a == None {
            unsafe { (*p.unwrap().as_ptr()).next = node_b };
            p = list_b.end;
        }
        if node_b == None {
            unsafe { (*p.unwrap().as_ptr()).next = node_a };
            p = list_a.end;
        }

        Self {
            length: len,
            start,
            end: p,
        }
    }
}

impl<T> Display for LinkedList<T>
    where
        T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
    where
        T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
