/*use std::cmp::Ordering;
use std::cmp::Ordering::{Less,Equal,Greater};*/

type Link<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
struct Node<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

pub struct Tree<T> {
    root: Link<T>,
}

impl <T: Ord> Node<T> {
    fn new_link(key: T) -> Link<T> {
        Some(Box::new(Node{key: key, left: None, right: None}))
    }

    fn search(&self, key: &T) -> bool {
        if *key == self.key {
            true
        } else if *key > self.key {
            match self.right {
                Some(ref right) => right.search(key),
                None => false,
            }
        } else {
            match self.left {
                Some(ref left) => left.search(key),
                None => false,
            }
        }
    }

    fn insert(&mut self, key: T) -> bool {
        if key == self.key {
            false
        } else if key > self.key {
            match self.right {
                Some(ref mut right) => right.insert(key),
                None => {
                    self.right = Node::new_link(key);
                    true
                },
            }
        } else {
            match self.left {
                Some(ref mut left) => left.insert(key),
                None => {
                    self.left = Node::new_link(key);
                    true
                },
            }
        }
    }

    fn preorder(&self) -> IterPreorder<T> {
        IterPreorder::new(self)
    }

    fn inorder(&self) -> IterInorder<T> {
        IterInorder::new(self)
    }

    fn postorder(&self) -> IterPostorder<T> {
        IterPostorder::new(self)
    }

}


impl<T: Ord> Tree<T> {
    pub fn new() -> Self {
        Tree{root: None}
    }

    pub fn insert(&mut self, key: T) -> bool {
        match self.root {
            Some(ref mut root) => root.insert(key),
            None => {
                self.root = Node::new_link(key);
                true
            },
        }
    }

    pub fn find(&self, key: &T) -> bool {
        match self.root {
            Some(ref head) => head.search(key),
            None => false,
        }
    }

    pub fn preorder(&self) -> IterPreorder<T> {
        match self.root {
            Some(ref root) => root.preorder(),
            None => IterPreorder::null(),
        }
    }

    pub fn inorder(&self) -> IterInorder<T> {
        match self.root {
            Some(ref root) => root.inorder(),
            None => IterInorder::null(),
        }
    }

    pub fn postorder(&self) -> IterPostorder<T> {
        match self.root {
            Some(ref root) => root.postorder(),
            None => IterPostorder::null(),
        }
    }
}

pub struct IterPreorder<'a, T: 'a> {
    waiting: Vec<&'a Node<T>>,
}

impl<'a, T> IterPreorder<'a, T> {
    fn new(root: &'a Node<T>) -> Self {
        let mut valid = IterPreorder{waiting: Vec::new()};
        valid.waiting.push(root);
        valid
    }

    fn null() -> Self {
        IterPreorder{waiting: Vec::new()}
    }
}

impl<'a, T> Iterator for IterPreorder<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.waiting.is_empty() {
            let top = self.waiting.pop().unwrap();
            match top.right {
                Some(ref right) => {self.waiting.push(right);},
                None => {},
            }
            match top.left {
                Some(ref left) => {self.waiting.push(left);},
                None => {},
            }
            Some(&top.key)
        } else {
            None
        }
    }
}

pub struct IterInorder<'a, T: 'a> {
    waiting: Vec<&'a Node<T>>,
    current: Vec<&'a Node<T>>,
}

impl<'a, T> IterInorder<'a, T> {
    fn new(root: &'a Node<T>) -> Self {
        let mut valid = IterInorder{
            waiting: Vec::new(),
            current: Vec::new(),
        };
        // valid.waiting.push(root);
        valid.current.push(root);
        valid
    }

    fn null() -> Self {
        IterInorder{
            waiting: Vec::new(),
            current: Vec::new(),
        }
    }
}

impl<'a, T> Iterator for IterInorder<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.current.len() == 1) | (!self.waiting.is_empty()) {
            while self.current.len() == 1 {
                let temp = self.current.pop().unwrap();
                match temp.left {
                    Some(ref left) => {self.current.push(left);},
                    None => {},
                }                
                self.waiting.push(temp);
            } /*then*/{
                let left_most = self.waiting.pop().unwrap();
                match left_most.right {
                    Some(ref right) => {self.current.push(right);},
                    None => {},
                }
                Some(&left_most.key)
            }
        } else {
            None
        }
    }

}



pub struct IterPostorder<'a, T: 'a> {
    waiting: Vec<&'a Node<T>>,
    pre: Option<&'a Node<T>>,
}

impl<'a, T> IterPostorder<'a, T> {
    fn new(root: &'a Node<T>) -> Self {
        let mut valid = IterPostorder{
            waiting: Vec::new(),
            pre: None,
        };
        let mut temp = root;
        valid.waiting.push(temp);
        loop {
            match temp.left {
                Some(ref left) => {
                    valid.waiting.push(left);
                    temp = left;
                },
                None => {break;},
            }
        }
        /*valid.waiting.push(temp);*/
        valid
    }

    fn null() -> Self {
        IterPostorder{
            waiting: Vec::new(),
            pre: None,
        }
    }
}

impl<'a, T:Ord> Iterator for IterPostorder<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut cur = None::<&'a Node<T>>;
        while (cur.is_some()) | (!self.waiting.is_empty()) {
            while cur.is_some() {
                let temp = cur.unwrap();
                self.waiting.push(temp);
                match temp.left {
                    Some(ref left) => {cur = Some(left);},
                    None => {cur = None;},
                }
            }
            cur = self.waiting.pop();
            let requirement: bool;
            if let Some(inside) = cur {
                match inside.right {
                    Some(ref right) => {
                        match self.pre {
                            Some(pre) => {
                                if pre.key == right.key {
                                    requirement = true;
                                } else {
                                    requirement = false;
                                }
                            },
                            None => {requirement = false;},
                        }
                    },
                    None => {requirement = true;},
                }
                if /*(temp.right.is_none()) | ((self.pre.is_some()) & (self.pre.unwrap().key == temp.right.unwrap().key)) */
                requirement {
                    self.pre = cur;
                    return Some(&self.pre.unwrap().key);
                } else {
                    self.waiting.push(inside);
                    match inside.right {
                        Some(ref right) => {cur = Some(right);},
                        None => {cur = None;},
                    }
                }
            }            
        }
        None
    } 
}