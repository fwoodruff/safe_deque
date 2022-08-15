
use std::sync::{Arc, Weak, Mutex};

struct Node<T> {
    value : T,
    next : Link<T>,
    previous : WeakLink<T>,
}

impl<T> Node<T> {
    pub fn new(val : T) -> Node<T> {
        Node { value: val, next: None, previous: Weak::new() }
    }
}


type WeakLink<T> = Weak<Mutex<Node<T>>>;
type Link<T> = Option<Arc<Mutex<Node<T>>>>;

pub struct Queue<T> {
    head : Link<T>,
    tail : WeakLink<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            head : None,
            tail : Weak::new(),
        }
    }

    pub fn push_tail(&mut self, val : T) {
        let new_node = Some(Arc::new(Mutex::new(Node::new(val))));
        let old_tail = self.tail.upgrade();
        self.tail = Arc::downgrade(new_node.as_ref().unwrap());
        match old_tail {
            None => {
                self.head = new_node;  
            },
            Some(link) => {
                link.lock().ok().unwrap().next = new_node.clone();
                let node = &mut new_node.as_ref().unwrap().lock().ok().unwrap();
                node.previous = Arc::downgrade(&link);
                
            }
        }
        
    }

    pub fn pop_head(&mut self) -> Option<T> {
        let link = self.head.take()?;
        self.head = link.lock().ok().unwrap().next.clone();
        Some(Arc::try_unwrap(link).ok().unwrap().into_inner().ok().unwrap().value)   
    }

    pub fn push_head(&mut self, val : T) {
        let new_node = Some(Arc::new(Mutex::new(Node{value: val, next : None, previous : Weak::new()})));
        
        let old_head = &self.head;
        match old_head {
            Some(v1) => {
                new_node.as_ref().unwrap().lock().ok().unwrap().next = Some(v1.clone());
                let v2 = &mut v1.lock().ok().unwrap().previous;
                *v2 = Arc::downgrade(new_node.as_ref().unwrap());
            }
            None => {
                self.tail = Arc::downgrade(&new_node.clone().unwrap());
            }
        }
        self.head = new_node;
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        let tl = self.tail.upgrade()?;
        
        let maybe_prev = tl.lock().ok().unwrap().previous.upgrade();
        
        match maybe_prev {
            None => {
                self.head = None;
                
                
            },
            Some(prv) => {
                prv.lock().ok().unwrap().next = None;
                
                
                self.tail = Arc::downgrade(&prv);
                
            }
        }
        let node_cell = Arc::try_unwrap(tl).ok().unwrap();
        Some(node_cell.into_inner().ok().unwrap().value)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_run() {
        let mut qq = Queue::new();
        qq.push_tail(5);
        qq.push_head(6);
        qq.push_tail(5);
        qq.pop_tail();
        qq.push_head(6);
        qq.pop_head();
        qq.pop_tail();
        qq.pop_head();

        
    }

    #[test]
    fn test_other() {

        
    }
}