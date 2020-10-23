/*

maximum_k_subarray のための機能しかないので、
必要になり次第追加して行く

*/

use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::rc::Rc;

use std::mem::replace;

pub struct LinkedList<T> {
    ghost: Rc<Node<T>>,
}

struct Node<T> {
    left: UnsafeCell<MaybeUninit<Rc<Node<T>>>>,
    right: UnsafeCell<MaybeUninit<Rc<Node<T>>>>,
    value: Option<T>,
}

pub struct Cursor<T> {
    ptr: Rc<Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        let ghost = Rc::new(Node {
            left: UnsafeCell::new(MaybeUninit::uninit()),
            right: UnsafeCell::new(MaybeUninit::uninit()),
            value: None,
        });
        unsafe {
            (*ghost.left.get()).as_mut_ptr().write(ghost.clone());
            (*ghost.right.get()).as_mut_ptr().write(ghost.clone());
        }
        Self { ghost }
    }

    pub fn cursor_front(&self) -> Cursor<T> {
        unsafe {
            Cursor {
                ptr: get_ref(&self.ghost.right).clone(),
            }
        }
    }

    pub fn cursor_back(&self) -> Cursor<T> {
        unsafe {
            Cursor {
                ptr: get_ref(&self.ghost.left).clone(),
            }
        }
    }

    pub fn push_back(&self, elt: T) {
        let elt = Rc::new(Node {
            left: UnsafeCell::new(MaybeUninit::uninit()),
            right: UnsafeCell::new(MaybeUninit::uninit()),
            value: Some(elt),
        });

        unsafe {
            let last = replace(&mut *self.ghost.left.get(), MaybeUninit::uninit()).assume_init();
            (*elt.right.get())
                .as_mut_ptr()
                .write(replace(get_mut(&last.right), elt.clone()));
            (*elt.left.get()).as_mut_ptr().write(last);
            (*self.ghost.left.get()).as_mut_ptr().write(elt);
        }
    }

    pub fn pop_front(&self) -> Option<T> {
        self.cursor_front().remove_current()
    }

    pub fn pop_back(&self) -> Option<T> {
        self.cursor_back().remove_current()
    }

    pub fn empty(&self) -> bool {
        unsafe { Rc::ptr_eq(get_ref(&self.ghost.left), &self.ghost) }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while !self.empty() {
            self.pop_front();
        }
        unsafe {
            replace(&mut *self.ghost.left.get(), MaybeUninit::uninit()).assume_init();
            replace(&mut *self.ghost.right.get(), MaybeUninit::uninit()).assume_init();
        }
    }
}

impl<T> Cursor<T> {
    pub fn current(&self) -> Option<&'_ T> {
        self.ptr.value.as_ref()
    }

    pub fn move_prev(&mut self) {
        unsafe {
            self.ptr = get_ref(&self.ptr.left).clone();
        }
    }

    pub fn move_next(&mut self) {
        unsafe {
            self.ptr = get_ref(&self.ptr.right).clone();
        }
    }

    pub fn insert_before(&self, item: T) {
        let elt = Rc::new(Node {
            left: UnsafeCell::new(MaybeUninit::uninit()),
            right: UnsafeCell::new(MaybeUninit::uninit()),
            value: Some(item),
        });
        unsafe {
            let last = replace(&mut *self.ptr.left.get(), MaybeUninit::uninit()).assume_init();
            (*elt.right.get())
                .as_mut_ptr()
                .write(replace(get_mut(&last.right), elt.clone()));
            (*elt.left.get()).as_mut_ptr().write(last);
            (*self.ptr.left.get()).as_mut_ptr().write(elt);
        }
    }

    pub fn remove_current(&mut self) -> Option<T> {
        if self.ptr.value.is_none() {
            None
        } else {
            assert_eq!(
                Rc::strong_count(&self.ptr),
                3,
                "removed while another Cursor was present"
            );
            unsafe {
                let left = replace(&mut *self.ptr.left.get(), MaybeUninit::uninit()).assume_init();
                let right =
                    replace(&mut *self.ptr.right.get(), MaybeUninit::uninit()).assume_init();
                *get_mut(&left.right) = right.clone();
                *get_mut(&right.left) = left;
                Rc::try_unwrap(replace(&mut self.ptr, right))
                    .ok()
                    .unwrap()
                    .value
            }
        }
    }
}

impl<T> PartialEq for Cursor<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.ptr, &other.ptr)
    }
}

impl<T> Eq for Cursor<T> {}

unsafe fn get_ref<T>(ptr: &UnsafeCell<MaybeUninit<T>>) -> &T {
    &*(*(ptr.get() as *const MaybeUninit<T>)).as_ptr()
}

unsafe fn get_mut<T>(ptr: &UnsafeCell<MaybeUninit<T>>) -> &mut T {
    &mut *(*ptr.get()).as_mut_ptr()
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

use std::clone::Clone;

impl<T> Clone for Cursor<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr.clone(),
        }
    }
}

#[test]
fn test_linked_list() {
    {
        let _l = LinkedList::<i32>::new();
    }
    let r = LinkedList::<i32>::new();
    r.push_back(1);
    r.pop_front();
}
