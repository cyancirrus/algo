use crate::tree::{Tree, Link};

impl <T> Tree <T> 
where T:Eq + PartialEq,
{
    fn equal(&self, base:&Link<T>, other:&Link<T>) -> bool {
        match (base, other) {
            (None, None) => return true,
            (Some(b), Some(o)) => {
                b.elem == o.elem
                    && self.equal(&b.left, &o.left)
                    && self.equal(&b.right, &o.right)
            }
            (_,_) => false,
        }
    }
}

impl<T> PartialEq for Tree<T> 
where T:Eq + PartialEq
{
    fn eq(&self, other:&Tree<T>) -> bool{
        self.equal(&self.root, &other.root)
    }

}
