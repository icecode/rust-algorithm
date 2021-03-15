use crate::LinkedNode;

pub struct LinkedList<T: Copy> {
    head: Option<Box<LinkedNode<T>>>
}


impl<T: Copy> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: Option::None,
        }
    }

    pub fn push_back(&mut self, elm: T) {
        let mut tail = &mut self.head;
        while tail.is_some() {
            tail = &mut tail.as_mut().unwrap().as_mut().next;
        }
        *tail = Option::Some(Box::new(LinkedNode { data: elm, next: None }))
    }

    pub fn push_front(&mut self, elm: T) {
        match self.head.take() {
            None => self.head = {
                Option::Some(Box::new(LinkedNode { data: elm, next: None }))
            },
            Some(box_val) => {
                self.head = Option::Some(Box::new(LinkedNode { data: elm, next: Option::from(box_val) }))
            }
        }
    }

    pub fn foreach(&mut self, apply: fn(e: &T)) {
        let mut node = &mut self.head;
        while node.is_some() {
            apply(&node.as_mut().unwrap().as_mut().data);
            node = &mut node.as_mut().unwrap().next;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        return if self.head.is_none() {
            None
        } else {
            let v = self.head.take().unwrap();
            self.head = v.next;
            Some(v.data)
        }
    }


    pub fn pop_back(&mut self) -> Option<T> {
        let mut tail = &mut self.head;
        while tail.is_some() && tail.as_mut().unwrap().next.is_some() {
            tail = &mut tail.as_mut().unwrap().next;
        }
        return if tail.is_some() {
            let data = match tail {
                Some(box_node) => Some(box_node.data),
                None => None
            };
            *tail = None;
            data
        } else {
            None
        }
    }
}