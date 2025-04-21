use std::rc::Rc;

#[allow(dead_code)]
type Link<T> = Option<Rc<Node<T>>>;

#[allow(dead_code)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[allow(dead_code)]
pub struct List<T> {
    head: Link<T>,
}


#[allow(dead_code)]
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}


impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref()
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.elem)
    }

    pub fn prepend(&self, elem: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> Self {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

}

impl <T> Drop for List <T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break
            }
        }
    }
}


impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(
            |node| {
                self.next = node.next.as_deref();
                &node.elem
            }
        )
    }
}
