use std::ptr::NonNull;
use std::marker::PhantomData;
// ranges

// binary could get out the linked list entry and could split
// could just be a linkedlist and then work on perhaps a tree
// the tree will need to have remove

type Link<T> = Option<NonNull<Node<T>>>;
type Point<T> = Option<NonNull<Range<T>>>;

struct Node<T>
where T: PartialOrd + Eq + PartialEq + Copy
{
    lower: T,
    upper: T,
    next: Link<T>,
    prev: Link<T>,
}

struct Range<T>
where T: PartialOrd + Eq + PartialEq + Copy
{
    bound:T,
    point:NonNull<Node<T>>,
    left:Point<T>,
    right:Point<T>,
}


struct RangeTree<T>
where T: PartialOrd + Eq + PartialEq + Copy
{
    len:usize,
    root:Point<T>,
}

trait BoundFunctions<T> {
    fn min(&self, other:&T) -> T;
    fn max(&self, other:&T) -> T;
}

impl <T: Ord + Copy> BoundFunctions<T> for T {
    #[inline] fn min(&self, other:&T) -> T { if *self <= *other { *self } else { *other }}
    #[inline] fn max(&self, other:&T) -> T { if *self >= *other { *self } else { *other }}
}

struct TreeCursor<T>
where T: PartialOrd + Eq + PartialEq + Copy
{
    tree:RangeTree<T>,
    list:LinkedList<T>,
}

impl <T> TreeCursor<T>
where T: PartialOrd + Eq + PartialEq + BoundFunctions<T> + Copy
{
    fn new() -> Self {
        Self {
            tree:RangeTree::new(),
            list:LinkedList::new(),
        }
    }
    fn insert(&mut self, range:(T,T)) {
        unsafe {
            let (mut lower, mut upper) = range;
            if self.list.head.is_none() {
                let node = self.list.push_head(range);        
                self.tree.insert(range.0, node);
            }

            let mut start = self.tree.find_pred(lower).or(self.list.head);
            if let Some(p) = start {
                if (*p.as_ptr()).upper < lower { start = (*p.as_ptr()).next; }
            }
            let mut c_node = start.and_then(|p| (*p.as_ptr()).next);
            let mut left_prev = None;
            while let Some(ptr) = c_node {
                let n = &*ptr.as_ptr();
                if n.upper < lower { left_prev = Some(ptr); break; }
                let prev = n.prev;
                lower = lower.min(&n.lower);
                upper = upper.min(&n.upper);
                self.list.detach(ptr);
                self.tree.remove_by_lower(n.lower);
                drop(Box::from_raw(ptr.as_ptr()));
                c_node = prev;
            }
            let mut right_next = None;
            c_node = start;
            while let Some(ptr) = c_node {
                let n = &*ptr.as_ptr();
                if n.lower > upper { right_next = Some(ptr); break; }
                let next = n.next;
                lower = lower.min(&n.lower);
                upper = upper.max(&n.upper);
                self.list.detach(ptr);
                self.tree.remove_by_lower(n.lower);
                drop(Box::from_raw(ptr.as_ptr()));
                c_node = next;
            }
            let merged_node = NonNull::new_unchecked(Box::into_raw(Box::new(
                Node {
                    lower,
                    upper,
                    next:left_prev,
                    prev:right_next,
            })));
            self.tree.insert(lower, merged_node);
        }
    }
}


impl <T> RangeTree<T>
where T: PartialOrd + Eq + PartialEq  + Copy
{
    fn new() -> Self {
        Self {
            len:0,
            root:None,
        }
    }
    fn find_pred(&self, bound:T) -> Option<NonNull<Node<T>>> {
        unsafe {
            let mut node = self.root;
            let mut res = None;
            while let Some(p) = node {
                let r = p.as_ref();
                if bound < r.bound {
                    node = r.left;
                } else {
                    res = Some(r.point);
                    node = r.right;
                }
            }
            res
        }
    }
    fn remove_by_lower(&mut self, bound:T) {
        if let Some(range) = self.remove(bound) {
            drop(range);
        }
    }

    fn get(&mut self, bound:T) -> Option<NonNull<Node<T>>> {
        unsafe {
            let mut node = &mut self.root;
            while let Some(n) = node {
                if bound < (*n.as_ptr()).bound{ node = &mut (*n.as_ptr()).left }
                else if bound > (*n.as_ptr()).bound { node = &mut (*n.as_ptr()).right }
                else { break; }
            }
            if let Some(n) = node {
                    return Some((*n.as_ptr()).point);
            }
        }
        None
    }
    fn insert(&mut self, bound:T, point:NonNull<Node<T>>)
    where T: Copy
    {
        unsafe {
            let mut node = &mut self.root;
            while let Some(ptr) = node {
                let r =  ptr.as_ref();
                if bound < r.bound  {
                    node = &mut (*ptr.as_ptr()).left;
                } else if bound > r.bound {
                    node = &mut (*ptr.as_ptr()).right;
                } else {
                    return;
                }
            }
            *node = Some(NonNull::new_unchecked(Box::into_raw(Box::new(
                Range {
                    bound,
                    point,
                    left:None,
                    right:None,
                }
            ))));
        }
    }
    fn remove(&mut self, bound:T) -> Option<Range<T>> {
        unsafe {
            let mut node = &mut self.root;
            while let Some(ptr) = node {
                let n = ptr.as_mut();
                if bound < n.bound  { node = &mut n.left; }
                else if n.bound < bound { node = &mut n.right; }
                else {
                    return Self::unlink(node);
                };
            }
            None
        }
    }
    fn unlink(link: &mut Option<NonNull<Range<T>>>) -> Option<Range<T>> {
        unsafe {
            let mut node = link.take()?;
            let node_ptr = node.as_mut();

            match (node_ptr.left.take(), node_ptr.right.take()) {
                (None, None) => {},
                (Some(child), None) | (None, Some(child)) => {
                    *link = Some(child);
                },
                (Some(left), Some(right)) => {
                    *link = Some(right);
                    let mut node = &mut Some(right);
                    while let Some(ptr) = node {
                        node = &mut (*ptr.as_ptr()).left;
                    }
                    *node = Some(left);
                }
            }
            Some(*Box::from_raw(node_ptr))
            // Some(node)
        }
    }
}


// cut left
// - l < cl && u < cu
// cut right
//  - cl < u && cu < u
// remove entirely
//  - l < cl && cu < u
// bisect into two
// - cl < l && u < cu 



struct LinkedList<T>
where T: Eq + PartialEq + PartialOrd + Copy
{
    len:usize,
    head:Link<T>,
    tail:Link<T>,
    _ghost:PhantomData<T>,
}

impl <T> LinkedList<T>
where T: Eq + PartialEq + PartialOrd + Copy
{
    fn new() -> Self {
        Self {
            len:0,
            head:None,
            tail:None,
            _ghost:PhantomData
        }
    }
    fn push_head(&mut self, range:(T, T)) -> NonNull<Node<T>> {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(
                Node {
                    lower:range.0,
                    upper:range.1,
                    next:None,
                    prev:None, 
                }
            )));
            if let Some(old) = self.head {
                (*new.as_ptr()).next = Some(old);
                (*old.as_ptr()).prev = Some(new);
            } else {
                self.tail = Some(new);
            }
            self.head = Some(new);
            self.len+=1;
            return new;
        }
    }
    fn push_tail(&mut self, range:(T, T)) -> NonNull<Node<T>> {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(
                Node {
                    lower:range.0,
                    upper:range.1,
                    prev:None,
                    next:None,
                }
            )));
            if let Some(old) = self.tail {
                (*new.as_ptr()).prev = Some(old);
                (*old.as_ptr()).next = Some(new);
            } else {
                self.head = Some(new);
            }
            self.tail = Some(new);
            self.len += 1;
            return new;
        }
    }
    fn pop_head(&mut self) -> Option<Node<T>> {
        unsafe {
            let node = self.head.take()?;
            let bnode = Box::from_raw(node.as_ptr());
            self.head = bnode.next;
            if let Some(new) = bnode.next {
                (*new.as_ptr()).prev = None;
            } else {
                self.tail = None
            }
            self.len -= 1;
            Some(*bnode)
        }
    }
    fn pop_tail(&mut self) -> Option<Node<T>> {
        unsafe {
            let node = self.tail.take()?;
            let bnode = Box::from_raw(node.as_ptr());
            self.tail = bnode.prev;
            if let Some(new) = bnode.prev {
                (*new.as_ptr()).next = None;
            } else {
                self.head = None;
            }
            self.len -= 1;
            Some(*bnode)
        }
    }
    fn detach(&mut self, node:NonNull<Node<T>>) -> NonNull<Node<T>> {
        unsafe {
            let pnode = (*node.as_ptr()).prev;
            let nnode = (*node.as_ptr()).next;
            match (pnode, nnode) {
                (Some(prev), Some(next)) => {
                    (*prev.as_ptr()).next = Some(next);
                    (*next.as_ptr()).prev = Some(prev);
                },
                (Some(prev), None) => {
                    (*prev.as_ptr()).next = None;
                    self.tail = Some(prev);
                },
                (None, Some(next)) => {
                    (*next.as_ptr()).prev = None;
                    self.head = Some(next);
                },
                (None, None) => {
                    self.head = None;
                    self.tail = None;
                }
            }
            (*node.as_ptr()).prev = None;
            (*node.as_ptr()).next = None;
            self.len -= 1;
            return node
        }
    }
    fn insert_between(&mut self, prev: Link<T>, next: Link<T>, range:(T, T)) -> NonNull<Node<T>> {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(
                Node {
                    lower:range.0,
                    upper:range.1,
                    prev:None,
                    next:None
                }
            )));
            match prev {
                Some(p) => { (*p.as_ptr()).next = Some(new); (*new.as_ptr()).prev = Some(p); }
                None    => { self.head = Some(new); }
            }
            match next {
                Some(n) => { (*n.as_ptr()).prev = Some(new); (*new.as_ptr()).next = Some(n); }
                None    => { self.tail = Some(new); }
            }
            self.len += 1;
            new
        }
    }
}

impl <T> Drop for LinkedList<T>
where T: Eq + PartialEq + PartialOrd + Copy
{
    fn drop(&mut self) {
        while let Some(_) = self.pop_head() {}
    }
}

fn main() {

}
