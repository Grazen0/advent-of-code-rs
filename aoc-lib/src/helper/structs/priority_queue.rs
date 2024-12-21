use std::{slice::Iter, vec::IntoIter};

#[derive(Debug, Clone, Default)]
pub struct PriorityQueue<T, I: PartialOrd>(Vec<(T, I)>);

impl<T, I: PartialOrd> PriorityQueue<T, I> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn insert(&mut self, value: T, priority: I) {
        let i = self.0.partition_point(|(_, p)| *p < priority);
        self.0.insert(i, (value, priority));
    }

    pub fn pop_value(&mut self) -> Option<T> {
        self.0.pop().map(|val| val.0)
    }

    pub fn pop(&mut self) -> Option<(T, I)> {
        self.0.pop()
    }

    pub fn peek(&self) -> Option<&(T, I)> {
        self.0.last()
    }

    pub fn peek_value(&self) -> Option<&T> {
        self.0.last().map(|val| &val.0)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> Iter<'_, (T, I)> {
        self.0.iter()
    }
}

impl<T: Eq, I: PartialOrd> PriorityQueue<T, I> {
    pub fn remove(&mut self, value: &T) -> bool {
        let Some(i) = self.0.iter().position(|e| e.0 == *value) else {
            return false;
        };

        self.0.remove(i);
        true
    }

    pub fn update_priority(&mut self, value: &T, new_priority: I) -> bool {
        let Some(i) = self.0.iter().position(|(item, _)| item == value) else {
            return false;
        };

        let el = self.0.remove(i);
        self.insert(el.0, new_priority);
        true
    }

    pub fn get_priority_of(&self, value: &T) -> Option<&I> {
        self.0
            .iter()
            .find(|(item, _)| item == value)
            .map(|item| &item.1)
    }
}

impl<T, I: PartialOrd> IntoIterator for PriorityQueue<T, I> {
    type Item = (T, I);
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Clone, Default)]
pub struct MinPriorityQueue<T, I: PartialOrd>(Vec<(T, I)>);

impl<T, I: PartialOrd> MinPriorityQueue<T, I> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn insert(&mut self, value: T, priority: I) {
        let i = self.0.partition_point(|(_, p)| *p > priority);
        self.0.insert(i, (value, priority));
    }

    pub fn pop_value(&mut self) -> Option<T> {
        self.0.pop().map(|val| val.0)
    }

    pub fn pop(&mut self) -> Option<(T, I)> {
        self.0.pop()
    }

    pub fn peek(&self) -> Option<&(T, I)> {
        self.0.last()
    }

    pub fn peek_value(&self) -> Option<&T> {
        self.0.last().map(|val| &val.0)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> Iter<'_, (T, I)> {
        self.0.iter()
    }
}

impl<T: Eq, I: PartialOrd> MinPriorityQueue<T, I> {
    pub fn remove(&mut self, value: &T) -> bool {
        let Some(i) = self.0.iter().position(|e| e.0 == *value) else {
            return false;
        };

        self.0.remove(i);
        true
    }

    pub fn update_priority(&mut self, value: &T, new_priority: I) -> bool {
        let Some(i) = self.0.iter().position(|(item, _)| item == value) else {
            return false;
        };

        let el = self.0.remove(i);
        self.insert(el.0, new_priority);
        true
    }

    pub fn get_priority_of(&self, value: &T) -> Option<&I> {
        self.0
            .iter()
            .find(|(item, _)| item == value)
            .map(|item| &item.1)
    }
}

impl<T, I: PartialOrd> IntoIterator for MinPriorityQueue<T, I> {
    type Item = (T, I);
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_insert_works() {
        let mut queue = PriorityQueue::new();

        queue.insert(1, 0);
        queue.insert(42, 4);
        queue.insert(30, 2);
        queue.insert(100, 1);
        queue.insert(76, 1);
        queue.insert(121, 3);

        assert_eq!(queue.pop_value(), Some(42));
        assert_eq!(queue.pop_value(), Some(121));
        assert_eq!(queue.pop_value(), Some(30));
        assert_eq!(queue.pop_value(), Some(100));
        assert_eq!(queue.pop_value(), Some(76));
        assert_eq!(queue.pop_value(), Some(1));
        assert_eq!(queue.pop_value(), None);
    }

    #[test]
    fn min_queue_insert_works() {
        let mut queue = MinPriorityQueue::new();

        queue.insert(1, 0);
        queue.insert(42, 4);
        queue.insert(30, 2);
        queue.insert(100, 1);
        queue.insert(76, 1);
        queue.insert(121, 3);

        assert_eq!(queue.pop_value(), Some(1));
        assert_eq!(queue.pop_value(), Some(100));
        assert_eq!(queue.pop_value(), Some(76));
        assert_eq!(queue.pop_value(), Some(30));
        assert_eq!(queue.pop_value(), Some(121));
        assert_eq!(queue.pop_value(), Some(42));
        assert_eq!(queue.pop_value(), None);
    }
}
