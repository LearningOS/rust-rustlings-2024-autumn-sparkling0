/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
         // 在堆末尾插入新值
         self.count += 1;
         if self.count == self.items.len() {
             self.items.push(value);
         } else {
             self.items[self.count] = value;
         }
 
         // 自下而上调整堆
         let mut idx = self.count;
         while idx > 1 {
             let parent_idx = self.parent_idx(idx);
             // 如果当前节点比父节点小，交换它们
             if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                 self.items.swap(idx, parent_idx);
                 idx = parent_idx;
             } else {
                 break;
             }
         }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		//0
         // 找到最小的子节点索引
         let left_child_idx = self.left_child_idx(idx);
         let right_child_idx = self.right_child_idx(idx);
 
         if right_child_idx <= self.count {
             // 如果有两个子节点，返回较小的一个
             if (self.comparator)(&self.items[right_child_idx], &self.items[left_child_idx]) {
                 return right_child_idx;
             }
         }
         left_child_idx
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default+Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		//None
        if self.is_empty() {
            return None;
        }

        // 取出根节点（最小或最大元素）
        let root_value = self.items[1].clone(); // Clone the root value
        self.count -= 1;
        // 将最后一个节点移动到根
        self.items[1] = self.items[self.count + 1].clone();
        self.items.pop(); // 去掉最后一个节点

        // 自上而下调整堆
        let mut idx = 1;
        while self.children_present(idx) {
            let smallest_child = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[smallest_child], &self.items[idx]) {
                self.items.swap(idx, smallest_child);
                idx = smallest_child;
            } else {
                break;
            }
        }

        Some(root_value)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}