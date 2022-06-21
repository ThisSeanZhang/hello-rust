use std::{rc::Rc, borrow::BorrowMut, cell::RefCell};

#[derive(Debug)]
pub struct SList {
  sentinel: IntNode,
  size: u64,
  head: Option<Box<RefCell<IntNode>>>
}

#[derive(Debug)]
struct IntNode {
  pub item: i64,
  pub next: Option<Box<RefCell<IntNode>>>
}

impl IntNode {
  fn new(item: i64, next: Option<IntNode>) -> IntNode {
    IntNode{
      item,
      next: if let Some(in_next) = next {
        Some(Box::new(RefCell::new(in_next)))
      } else {
        None
      }
    }
  }
}

impl SList {

  fn new(item_op: Option<i64>) -> SList {
    match item_op {
      Some(item) => SList {
        sentinel: IntNode { item: i64::MAX, next: Some(Box::new(RefCell::new(IntNode{ item, next: None }))) },
        size: 1,
        head: Some(Box::new(RefCell::new(IntNode { item: i64::MAX, next: Some(Box::new(RefCell::new(IntNode{ item, next: None }))) })))
      },
      None => SList {
        sentinel: IntNode { item: i64::MAX, next: None },
        size: 0,
        head: Some(Box::new(RefCell::new(IntNode { item: i64::MAX, next: None })))
      }
    }
  }

  fn add_first(&mut self, item: i64) {
    self.size += 1;
    self.sentinel.next = Some(Box::new(RefCell::new(IntNode {
      item,
      next: self.sentinel.next.take()
    })));
    if let Some(head) = &mut self.head {
      head.as_mut().get_mut().next = Some(Box::new(RefCell::new(IntNode {
        item,
        next: head.as_mut().get_mut().next.take()
      })));
    }
  }

  fn get_first(& self) ->  Option<i64> {
    // self.sentinel.next.as_ref().unwrap().borrow().item;
    if let Some(item) = &self.head.as_ref().unwrap().borrow().next {
      return Some(item.borrow().item);
    }
    None
  }

  fn get_inner(list: &Option<Box<RefCell<IntNode>>>, index: u64) -> Option<i64> {
    if index == 1 {
      Some(list.as_ref().unwrap().borrow().item)
    } else {
      SList::get_inner(&list.as_ref().unwrap().borrow().next, index - 1)
    }
  }

  fn get(&self, index: u64) -> Option<i64> {
    SList::get_inner(&self.head.as_ref().unwrap().as_ref().borrow().next, index)
  }

  fn add_last(&mut self, item: i64) {
    self.size += 1;
    let mut p = &mut self.head;
 		while let Some(in_next) = p {
      // let a = p.unwrap();
      if in_next.as_ref().borrow().next.is_some() {
        p = &mut in_next.as_mut().get_mut().next;
      } else {
        in_next.as_mut().get_mut().next = Some(Box::new(RefCell::new(IntNode {
          item,
          next: None
        })));
        break;
      }
 		}
  }

  fn size(&self) -> u64 {
    self.size
  }

}

#[cfg(test)]
mod tests {
    use crate::s_list::SList;

  #[test]
  fn size_test() {
    let mut list = SList::new(Some(12));
    list.add_last(42);
    println!("{:?}", list);
    assert_eq!(list.size(), 2);
  }

  #[test]
  fn when_slist_empty_add_first() {
    let mut list = SList::new(None);
    list.add_first(42);
    println!("{:?}", list);
    assert_eq!(list.size(), 1);
  }

  #[test]
  fn when_slist_empty_add_first_twice() {
    let mut list = SList::new(None);
    list.add_first(42);
    println!("{:?}", list);
    list.add_first(64);
    println!("{:?}", list);
    assert_eq!(list.size(), 2);
  }

  #[test]
  fn when_slist_empty_add_first_twice_and_get_first() {
    let mut list = SList::new(None);
    list.add_first(42);
    println!("{:?}", list);
    list.add_first(64);
    println!("{:?}", list);
    assert_eq!(list.get_first(), Some(64));
    // get twice
    assert_eq!(list.get_first(), Some(64));
  }

  #[test]
  fn when_slist_empty_add_first_twice_and_get_index() {
    let mut list = SList::new(None);
    list.add_first(42);
    list.add_first(64);
    assert_eq!(list.get(1), Some(64));
    println!("{:?}", list);
    // get twice
    assert_eq!(list.get(2), Some(42));
    println!("{:?}", list);
  }

  #[test]
  fn when_slist_empty_add_last() {
    let mut list = SList::new(None);
    list.add_last(42);
    println!("{:?}", list);
    assert_eq!(list.size(), 1);
  }

  #[test]
  fn when_slist_empty_add_last_twice() {
    let mut list = SList::new(None);
    list.add_last(42);
    list.add_last(64);
    println!("{:?}", list);
    assert_eq!(list.size(), 2);
  }
}