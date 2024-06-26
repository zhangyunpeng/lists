use std::sync::Arc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Arc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None}
    }
    pub fn prepend(&mut self, elem: T) -> List<T> {
        List {
            head: Some(Arc::new(Node{
                elem,
                next: self.head.clone(),
            })),
        }
    }
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone())
        }
    }
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node|&node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(node) = cur {
            if let Ok(node) = Arc::try_unwrap(node) {
                cur = node.next;
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref().map(|node|node),
        }
    }
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node|{
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}