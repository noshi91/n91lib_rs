/*

References

[1] van Leeuwen, J., & Wood, D. (1993).
    Interval heaps. The Computer Journal, 36(3), 209-216.


Description

概要は以下のページを参照せよ

https://scrapbox.io/data-structures/Interval_Heap

[1] をあまり読み込んでいないため、
いくらか異なる実装をしている可能性がある。

*/

use std::cmp::Ord;

pub struct IntervalHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> IntervalHeap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn pop_min(&mut self) -> Option<T> {
        let len = self.data.len();
        match len {
            0 => None,
            1 => self.data.pop(),
            _ => {
                self.data.swap(0, len - 1);
                let ret = self.data.pop();
                cascade_down_start(&mut self.data); // may bug
                ret
            }
        }
    }

    pub fn pop_max(&mut self) -> Option<T> {
        let len = self.data.len();
        match len {
            0 => None,
            1 => self.data.pop(),
            _ => {
                self.data.swap(1, len - 1);
                let ret = self.data.pop();
                cascade_down_end(&mut self.data);
                ret
            }
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        let index = self.data.len() - 1;
        if index % 2 == 0 {
            if index == 0 {
                return;
            }
            let p_end = index / 2 - 1 | 1;
            if self.data[p_end] < self.data[index] {
                self.data.swap(p_end, index);
                cascade_up_end(&mut self.data, p_end);
            } else {
                cascade_up_start(&mut self.data, index);
            }
        } else {
            if self.data[index - 1] > self.data[index] {
                self.data.swap(index - 1, index);
                cascade_up_start(&mut self.data, index - 1);
            } else {
                cascade_up_end(&mut self.data, index);
            }
        }
    }

    pub fn peek_min(&self) -> Option<&T> {
        match self.len() {
            0 => None,
            _ => Some(&self.data[0]),
        }
    }

    pub fn peek_max(&self) -> Option<&T> {
        match self.len() {
            0 => None,
            1 => Some(&self.data[0]),
            _ => Some(&self.data[1]),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

fn cascade_down_start<T: Ord>(data: &mut [T]) {
    let len = data.len();
    let mut index = 0;
    loop {
        let end = index + 1;
        if end >= len {
            break;
        }
        if data[index] > data[end] {
            data.swap(index, end);
        }
        let left = index * 2 + 2;
        if left >= len {
            break;
        }
        let right = index * 2 + 4;
        let next = if right >= len || data[left] < data[right] {
            left
        } else {
            right
        };
        if data[index] <= data[next] {
            break;
        }
        data.swap(index, next);
        index = next;
    }
}

fn cascade_down_end<T: Ord>(data: &mut [T]) {
    let len = data.len();
    let mut index = 1;
    loop {
        let start = index - 1;
        if data[start] > data[index] {
            data.swap(start, index);
        }
        let left = index * 2 + 1;
        if left >= len {
            break;
        }
        let right = index * 2 + 3;
        let next = if right >= len || data[left] > data[right] {
            left
        } else {
            right
        };
        if data[index] >= data[next] {
            break;
        }
        data.swap(index, next);
        index = next;
    }
}

fn cascade_up_start<T: Ord>(data: &mut [T], mut index: usize) {
    while index != 0 {
        let parent = index / 2 - 1 & !1;
        if data[parent] <= data[index] {
            break;
        }
        data.swap(parent, index);
        index = parent;
    }
}

fn cascade_up_end<T: Ord>(data: &mut [T], mut index: usize) {
    while index != 1 {
        let parent = index / 2 - 1 | 1;
        if data[parent] >= data[index] {
            break;
        }
        data.swap(parent, index);
        index = parent;
    }
}

#[test]
fn interval_heap_test() {
    let mut ih = IntervalHeap::<i32>::new();
    assert_eq!(ih.peek_max(), None);
    ih.push(-1);
    ih.push(1);
    ih.push(0);
    assert_eq!(ih.peek_max(), Some(&1));
    assert_eq!(ih.peek_min(), Some(&-1));
    assert_eq!(ih.len(), 3);
    ih.pop_max();
    assert_eq!(ih.peek_max(), Some(&0));
    ih.pop_min();
    assert_eq!(ih.peek_min(), Some(&0));
}
