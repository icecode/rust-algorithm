

pub struct ArrayList<T: Copy> {
    elms: Vec<T>,
}

impl<T: Copy> ArrayList<T> {
    pub fn new() -> ArrayList<T> {
        ArrayList {
            elms: Vec::with_capacity(2)
        }
    }

    pub fn foreach(&self, apply: fn(e: &T)) {
        for i in 0..self.elms.len() {
            let el = &self.elms[i];
            apply(el)
        }
    }

    pub fn push(&mut self, elm: T) -> bool {
        self.try_grow();
        self.elms.push(elm);
        true
    }

    pub fn add_to_index(&mut self, index: usize, elm: T) -> bool {
        if self.elms.len() < index {
            return false;
        }
        self.try_grow();
        self.elms.insert(index, elm);
        return true;
    }

    pub fn remove_to_index(&mut self, index: usize) -> T {
        self.elms.remove(index)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elms.pop()
    }

    pub fn try_grow(&mut self) {
        if (self.elms.len() + 1) >= self.elms.capacity() {
            self.elms.reserve(self.elms.capacity());
        }
    }
}