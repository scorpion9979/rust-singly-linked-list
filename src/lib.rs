pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        SimpleLinkedList { head: None }
    }
}

impl<T: Copy> Clone for SimpleLinkedList<T> {
    fn clone(&self) -> SimpleLinkedList<T> {
        let mut out: SimpleLinkedList<T> = SimpleLinkedList::new();
        let mut cur = &self.head;
        while let Some(node) = cur {
            cur = &node.next;
            out.push(node.data)
        }
        out
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        let mut c = 0;
        let mut cur = &self.head;
        while let Some(node) = cur {
            cur = &node.next;
            c += 1;
        }
        c
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, _element: T) {
        let mut cur = &mut self.head;
        match cur {
            Some(_) => {
                while let Some(node) = cur {
                    cur = &mut node.next;
                }
            }
            None => (),
        }
        *cur = Some(Box::from(Node {
            data: _element,
            next: None,
        }));
    }

    pub fn pop(&mut self) -> Option<T>
    where
        T: Copy,
    {
        let length = &self.len();
        let mut cur = &mut self.head;
        let mut out = None;
        match cur {
            Some(_) if *length > 1usize => {
                let mut c = 0usize;
                while let Some(node) = cur {
                    cur = &mut node.next;
                    if c >= length - 1 {
                        out = Some(node.data);
                        break;
                    }
                    c += 1;
                }

                c = 0usize;
                cur = &mut self.head;
                while let Some(node) = cur {
                    cur = &mut node.next;
                    if c == length - 2 {
                        break;
                    }
                    c += 1;
                }
            }
            Some(node) => out = Some(node.data),
            None => (),
        }
        *cur = None;
        out
    }

    pub fn peek(&self) -> Option<&T> {
        let cur = &self.head;
        match cur {
            Some(node) => Some(&node.data),
            None => None,
        }
    }
}

impl<T: Copy> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut clone = self.clone();
        let mut out: SimpleLinkedList<T> = SimpleLinkedList::new();
        while let Some(val) = clone.pop() {
            out.push(val)
        }
        out
    }
}

impl<'a, T: Copy> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        let mut out: SimpleLinkedList<T> = SimpleLinkedList::new();
        for &e in _item.iter() {
            out.push(e);
        }
        out
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut out: Vec<T> = Vec::new();
        let mut cur = self.head;
        while let Some(node) = cur {
            cur = node.next;
            out.push(node.data)
        }
        out
    }
}
