#![no_std]

use core::marker::PhantomData;
use core::ptr::NonNull;

pub struct ListItem<'a, T> {
    value: T,
    next: Option<NonNull<ListItem<'a, T>>>,
    marker: PhantomData<&'a ListItem<'a, T>>,
    id: u32,
    pub priority: u32,
}

enum TaskState {
    SLEEP,
    READY,
    ACTIVE,
}

pub struct LinkedList<'a, T> {
    head: Option<NonNull<ListItem<'a, T>>>,
    last: Option<NonNull<ListItem<'a, T>>>,
    marker: PhantomData<&'a ListItem<'a, T>>,
    priority: u32,
}

use core::ops::{Deref, DerefMut};

impl<'a, T> Deref for ListItem<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, T> DerefMut for ListItem<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<'a, T> ListItem<'a, T> {
    pub fn new(value: T, priority: u32, id: u32) -> Self {
        ListItem {
            value,
            next: None,
            marker: PhantomData,
            id,
            priority,
        }
    }
}

impl<'a, T> LinkedList<'a, T> {
    pub fn new(priority: u32) -> Self {
        LinkedList {
            head: None,
            last: None,
            marker: PhantomData,
            priority,
        }
    }

    pub fn push(&mut self, item: &'a mut ListItem<'a, T>) {
        let ptr = unsafe { NonNull::new_unchecked(item as *mut ListItem<T>) };
        let prev_last = self.last.replace(ptr);

        if prev_last.is_none() {
            self.head = Some(ptr);
        } else {
            prev_last.map(|mut i| unsafe {
                i.as_mut().next = Some(ptr);
            });
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn head_mut(&mut self) -> Option<&mut T> {
        self.head
            .map(|ptr| unsafe { &mut *ptr.as_ptr() }.deref_mut())
    }

    pub fn pop(&mut self) -> Option<&'a mut ListItem<'a, T>> {
        let result = self.head.take();
        let next = result.and_then(|mut ptr| unsafe { ptr.as_mut().next });

        if next.is_none() {
            self.last = None;
        }

        self.head = next;

        result.map(|ptr| unsafe { &mut *ptr.as_ptr() })
    }
}

#[cfg(test)]
mod test {
    use LinkedList;
    use ListItem;

    #[test]
    fn test_list() {
        let mut item1 = ListItem::new(1, 1, 1);
        let mut item2 = ListItem::new(2, 2, 2);
        let mut item3 = ListItem::new(3, 3, 3);
        let mut item4 = ListItem::new(4, 2, 4);
        let mut list1 = LinkedList::new(1);
        let mut list2 = LinkedList::new(2);
        let mut list3 = LinkedList::new(3);

        list.push(&mut item1);
        list.push(&mut item2);
        list.push(&mut item3);

        assert_eq!(Some(&mut 1), list.head_mut());
        let result1: &u32 = list.pop().unwrap();
        assert_eq!(Some(&mut 2), list.head_mut());
        let result2: &u32 = list.pop().unwrap();
        assert_eq!(Some(&mut 3), list.head_mut());
        let result3: &u32 = list.pop().unwrap();
        assert_eq!(1, *result1);
        assert_eq!(2, *result2);
        assert_eq!(3, *result3);

        assert!(list.is_empty());

        let mut item4 = ListItem::new(4);
        let mut item5 = ListItem::new(5);
        list.push(&mut item4);
        list.push(&mut item5);

        let result4: &u32 = list.pop().unwrap();
        let result5: &u32 = list.pop().unwrap();
        assert_eq!(4, *result4);
        assert_eq!(5, *result5);

        assert!(list.is_empty());
    }
}
