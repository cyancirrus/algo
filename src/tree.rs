use std::fmt::Debug;
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

pub struct Tree<T> {
    root: Link<T>,
}

pub struct Iter<T> {
    next: Link<T>, 
}

impl <T:PartialOrd>  Tree<T> {
    pub fn insert(&mut self, elem:T) {
        let mut curr_node = &mut self.root;
        while let Some(node) = curr_node {
            if elem < node.elem {
                curr_node = &mut node.left;
            } else {
                curr_node = &mut node.right;
            }
        }
        *curr_node = Some(Box::new(Node {
            elem,
            left: None,
            right:None,
        }));
    }
    pub fn min(&self) -> Option<&T> {
        let mut curr_node = &self.root;
        while let Some(node) = curr_node {
            curr_node = &node.left;
        }
        curr_node.as_ref().map(|node| &node.elem)
    }
    pub fn max(&self) -> Option<&T> {
        let mut curr_node = &self.root;
        while let Some(node) = curr_node {
            curr_node = &node.right;
        }
        curr_node.as_ref().map(|node| &node.elem)
    }
    pub fn contains(&self, elem:T) -> bool {
        let mut curr_node = &self.root;
        while let Some(node) = curr_node {
            if elem < node.elem {
                curr_node = &node.left;
            } else if elem > node.elem {
                curr_node = &node.right;
            } else {
                return true
            }
        }
        false
    }
    pub fn floor_strict(&self, elem:T) -> Option<&T> {
        let mut val = None;
        let mut curr_node = &self.root;
        while let Some(node) = curr_node {
            if elem < node.elem {
                curr_node = &node.left;
            } else if elem > node.elem {
                val = Some(&node.elem);
                curr_node = &node.right;
            } else {
                // find the minimimum closest value even if we match
                curr_node = &node.left;
            }
        }
        val
    }
    pub fn ceiling_strict(&self, elem:T) -> Option<&T> {
        let mut val = None;
        let mut curr_node = &self.root;
        while let Some(node) = curr_node {
            if elem < node.elem {
                val = Some(&node.elem);
                curr_node = &node.left;
            } else if elem > node.elem {
                curr_node = &node.right;
            } else {
                // find the minimimum closest value even if we match
                curr_node = &node.right;
            }
        }
        val
    }
}

impl<T> Tree<T> {
    pub fn into_iter(self) -> Iter<T> {
        Iter{ next:self.root }
    }

}
impl <T:Debug> Tree<T> {    
    pub fn display(&self) {
            self._display(&self.root);
    }
    fn _display(&self, curr_node:&Link<T>) {
        if let Some(node) = curr_node {
            self._display(&node.left);
            println!("Node elem {:?}", node.elem);
            self._display(&node.right);
        }
    }
}
// impl<T> Iter<T> {
//     fn next(&mut self) -> Option<&T> {
//         let val = self.next.map(|node| &node.elem);
//         self.next =

//         todo!()
//     }
// }
