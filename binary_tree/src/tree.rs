type Link<T> = Option<Box<Node<T>>>;
/*#[derive(Clone)]*/
struct Node<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

/*create a mutable Link, since Rust does not support NULL ptr, there is no 
need for handling NULL parameter*/

impl<T: Ord/* + Clone*/> Node<T> {
    fn new_link(key: T) -> Link<T> {
        let /*mut*/ t = Some(Box::new(Node{key:key, left:None, right:None}));
        t
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

    fn insert(& mut self, key:T) -> bool {
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
    
    fn preorder<'a, 'b>(&'a self, pre_vec: &'b mut Vec<&'a T>) -> &'b Vec<&'a T> {
        pre_vec.push(&self.key/*.clone()*/);
        match self.left {
            Some(ref left) => {left.preorder(pre_vec);}
            None => {},
        }
        match self.right {
            Some(ref right) => {right.preorder(pre_vec);}
            None => {},
        }
        pre_vec
    }

    fn inorder<'a, 'b>(&'a self, in_vec: &'b mut Vec<&'a T>) -> &'b Vec<&'a T> {
        match self.left {
            Some(ref left) => {left.inorder(in_vec);}
            None => {},
        }
        in_vec.push(&self.key/*.clone()*/);
        match self.right {
            Some(ref right) => {right.inorder(in_vec);}
            None => {},
        }
        in_vec
    }

    fn postorder<'a, 'b>(&'a self, post_vec: &'b mut Vec<&'a T>) -> &'b Vec<&'a T> {
        match self.left {
            Some(ref left) => {left.postorder(post_vec);}
            None => {},
        }
        match self.right {
            Some(ref right) => {right.postorder(post_vec);}
            None => {},
        }
        post_vec.push(&self.key/*.clone()*/);
        post_vec
    }
}


/*#[derive(Clone)]*/
pub struct Tree<T> {
    root: Link<T>,
}

impl<T: Ord/* + Clone*/> Tree<T> {
    /// Creates an empty tree: which there is no Node inside
    pub fn new() -> Self {
        let /*mut*/ t = Tree{root: None};
        t
    }

    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {
        match self.root {
            Some(ref mut root) => root.insert(key),
            None => {
                self.root = Node::new_link(key);
                true
            },
        }   
    }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        match self.root {
            Some(ref h) => h.search(key),
            None => false,
        }
    }
    

    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
        let mut pre_vec: Vec<&T> = Vec::new();
        match self.root {
            Some(ref root) => {
                root.preorder(&mut pre_vec);
            },  
            None => {},
        }
        pre_vec
    }

    /// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        let mut in_vec: Vec<&T> = Vec::new();
        match self.root {
            Some(ref root) => {
                root.inorder(&mut in_vec);
            },  
            None => {},
        }
        in_vec
    }

    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        let mut post_vec: Vec<&T> = Vec::new();
        match self.root {
            Some(ref root) => {
                root.postorder(&mut post_vec);
            },  
            None => {},
        }
        post_vec
    }
}
