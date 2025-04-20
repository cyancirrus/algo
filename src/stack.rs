use std::rc::Rc;

#[allow(dead_code)]
type Link<T> = Option<Rc<Node<T>>>;

#[allow(dead_code)]
struct Node<T> {
    elem:T,
    next:Link<T>,
}

#[allow(dead_code)]
pub struct List <T> {
    head:Link<T>,
}

impl <T> List<T> {
    pub fn new() -> Self {
        List { head: None } 
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.elem)
    }

    pub fn prepend(&self, elem: T) -> Self {
        List { head: Some(Rc::new(
                Node { elem: elem, next: self.head.clone() }
            ))
        }
    }

    pub fn tail(&self) -> Self {
        List {
            head: self.head.as_ref()
                .and_then( |node| node.next.clone() )
        }
    }
}


