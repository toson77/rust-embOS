use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

#[repr(C)]
pub struct ListItem<'a, T> {
    pub item: T,
    next: Option<&'a mut ListItem<'a, T>>,
    prev: Option<NonNull<ListItem<'a, T>>>,
}

pub struct LinkedList<'a, T> {
    head: Option<&'a mut ListItem<'a, T>>,
    last: Option<NonNull<ListItem<'a, T>>>,
    len: usize,
}

pub struct Iter<'a, T> {
    head: Option<*const ListItem<'a, T>>,
    len: usize,
}

pub struct IterMut<'a, T> {
    head: Option<NonNull<ListItem<'a, T>>>,
    len: usize,
}

impl<'a, T> Deref for ListItem<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<'a, T> DerefMut for ListItem<'a, T> {
    fn deref_mut(&mut self) -> &mut self::Target {
        &mut self.item
    }
}

impl<'a, T> LinkedList<'a, T> {
    pub fn new() -> LinkedList<'a, T> {
        LinkedList {
            head: None,
            last: None,
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn pop(&mut self) -> Option<&'a mut ListItem<'a, T>> {
        let next = match self.head.iter_mut().next() {
            Some(item) => {
                let mut result = (*item).next.take();
                match result.iter_mut().next() {
                    Some(item) => {
                        (*item).prev = None;
                    }
                    None => {}
                }
                result
            }
            None => {
                panic!("empty list");
            }
        };
        let result = self.head.take();
        if next.is_none() {
            self.last = None;
        }
        self.head = next;
        self.len -= 1;
        result
    }

    pub fn push(&mut self, item: &'a mut ListItem<'a, T>) {
        if self.last.is_none() {
            let item_ptr = unsafe { NonNull::new_unchecked(item as *mut ListItem<T>) };
            item.prev = None;
            item.next = None;
            self.last.replace(item_ptr);
            self.head.replace(item);
        } else {
            let mut last_ptr = self.last.unwrap();
            let item_ptr = unsafe { NonNull::new_unchecked(item as *mut ListItem<T>) };
            self.last.replace(item_ptr);
            item.prev.replace(last_ptr);
            unsafe {
                last_ptr.as_mut().next.replace(item);
            }
        }
        self.len += 1;
    }

    pub fn head_mut(&mut self) -> Option<&mut &'a mut ListItem<'a, T>) {
        self.head.as_mut()
    }

    pub fn join(&mut self, target: &mut LinkedList<'a, T>) {
        if self.is_empty() {
            self.head = target.head.take();
            self.last = target.last.take();
        } else {
            let mut tar_head = target.head.take();
            if tar_head.is_some() {
                let tar_head_item = tar_head.iter_mut().next().unwrap();
                let mut last_ptr = self.last.unwrap();
                unsafe {
                    last_ptr.as_mut().next = tar_head;
                }
                self.last = target.last.take();
            }
        }
        self.len += target.len;
        target.len = 0;
    }

    pub fn iter(&self) -> Iter<'a, T> {
        Iter {
            head: self.head.as_ref().map(|item| *item as *const ListItem<T>),
            len: self.len
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'a, T> {
        IterMut {
            head: self.head.as_mut().map(|item| unsafe {NonNull::new_unchecked(*item as *mut ListItem<T>)}),
            len: self.len,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|item| unsafe {
                let node = &(*item);
                self.len -= 1;
                self.head = node.next.as_ref().map(|item| *item as *const ListItem<T>);
                &node.item
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|ptr| unsafe {
                let node = &mut *ptr.as_ptr();
                self.len -= 1;
                self.head = node.next.as_mut().map(|item| NonNull::new_unchecked(*item as *mut ListItem<T>));
                &mut node.item
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> ListItem<'a, T> {
    pub fn crate(item: T) -> ListItem<'a, T> {
        ListItem {
            item,
            next: None,
            prev: None,
        }
    }
}
