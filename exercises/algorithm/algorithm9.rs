/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM DONE

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
        self.count += 1;
        self.items.push(value);
        let mut new_idx = self.count;
        let mut parent_idx = self.parent_idx(new_idx);
        let mut flag_finish = false;
        while parent_idx > 0 && !flag_finish {
            let par_value = self.items.get(parent_idx).unwrap();
            let new_val = self.items.get(new_idx).unwrap();
            if (self.comparator)(new_val, par_value){
                let tmp_val = &mut T::default();
                std::mem::swap(&mut self.items[parent_idx], tmp_val);
                std::mem::swap(&mut self.items[new_idx], tmp_val);
                std::mem::swap(&mut self.items[parent_idx], tmp_val);
                (new_idx, parent_idx) = (parent_idx, self.parent_idx(parent_idx));
            }else{
                flag_finish = true
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
		0
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
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        match self.len(){
            0 => None,
            1 => {
                self.count -= 1;
                self.items.pop()
            },
            n => {
                self.count -= 1;
                let mut rlt = T::default();
                let rlt_val = &mut rlt;
                std::mem::swap(&mut self.items[1], rlt_val);
                let mut tmp_val = self.items.pop().unwrap();
                let mut flag_finish = false;
                let mut par_idx = 1_usize;
                std::mem::swap(&mut self.items[par_idx], &mut tmp_val);
                while !flag_finish {
                    let par_val = self.items.get(par_idx).unwrap();
                    let left_child_idx = self.left_child_idx(par_idx);
                    let right_child_idx = self.right_child_idx(par_idx);
                    match self.items.get(left_child_idx){
                        None => {
                            flag_finish = true;
                        },
                        Some(left_val) => {
                            match self.items.get(right_child_idx){
                                None => {
                                    if (self.comparator)(par_val, left_val){
                                        flag_finish = true;
                                    }else{
                                        std::mem::swap(&mut self.items[left_child_idx], &mut tmp_val);
                                        std::mem::swap(&mut self.items[par_idx], &mut tmp_val);
                                        std::mem::swap(&mut self.items[left_child_idx], &mut tmp_val);
                                        par_idx = left_child_idx;
                                    }
                                },
                                Some(right_val) => {
                                    if (self.comparator)(left_val, right_val){
                                        if (self.comparator)(par_val, left_val){
                                            flag_finish = true;
                                        }else{
                                            std::mem::swap(&mut self.items[left_child_idx], &mut tmp_val);
                                            std::mem::swap(&mut self.items[par_idx], &mut tmp_val);
                                            std::mem::swap(&mut self.items[left_child_idx], &mut tmp_val);
                                            par_idx = left_child_idx;
                                        }
                                    }else{
                                        if (self.comparator)(par_val, right_val){
                                            flag_finish = true;
                                        }else{
                                            std::mem::swap(&mut self.items[right_child_idx], &mut tmp_val);
                                            std::mem::swap(&mut self.items[par_idx], &mut tmp_val);
                                            std::mem::swap(&mut self.items[right_child_idx], &mut tmp_val);
                                            par_idx = right_child_idx;
                                        }
                                    }       
                                }
                            }
                        }
                    }
                }
                Some(rlt)
            }
        }
        
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