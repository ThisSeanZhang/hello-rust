#[derive(Debug)]
pub struct SList {
  size: u64,
  sentinel: Option<Box<IntNode>>
}

#[derive(Debug)]
struct IntNode {
  pub item: i64,
  pub next: Option<Box<IntNode>>
}

impl IntNode {
  fn new(item: i64, next: Option<IntNode>) -> IntNode {
    IntNode{
      item,
      next: if let Some(in_next) = next {
        Some(Box::new(in_next))
      } else {
        None
      }
    }
  }
}

impl SList {

  pub fn new(item_op: Option<i64>) -> SList {
    match item_op {
      Some(item) => SList {
        size: 1,
        sentinel: Some(Box::new(IntNode { item: i64::MAX, next: Some(Box::new(IntNode{ item, next: None })) }))
      },
      None => SList {
        size: 0,
        sentinel: Some(Box::new(IntNode { item: i64::MAX, next: None }))
      }
    }
  }

  pub fn add_first(&mut self, item: i64) {
    self.size += 1;
    if let Some(head) = &mut self.sentinel {
      let next = head.as_mut().next.take();
      head.as_mut().next = Some(Box::new(IntNode {
        item,
        next
      }));
    }
  }

  fn get_first(& self) ->  Option<i64> {
    // self.sentinel.next.as_ref().unwrap().borrow().item;
    if let Some(item) = &self.sentinel.as_ref().unwrap().next {
      return Some(item.item);
    }
    None
  }

  fn get_inner(list: &Option<Box<IntNode>>, index: u64) -> Option<i64> {
    if index == 1 {
      Some(list.as_ref().unwrap().item)
    } else {
      SList::get_inner(&list.as_ref().unwrap().next, index - 1)
    }
  }

  fn get(&self, index: u64) -> Option<i64> {
    SList::get_inner(&self.sentinel.as_ref().unwrap().as_ref().next, index)
  }

  fn add_last(&mut self, item: i64) {
    self.size += 1;
    let mut p = &mut self.sentinel;
 		while let Some(in_next) = p {
      // let a = p.unwrap();
      if in_next.as_ref().next.is_some() {
        p = &mut in_next.as_mut().next;
      } else {
        in_next.as_mut().next = Some(Box::new(IntNode {
          item,
          next: None
        }));
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