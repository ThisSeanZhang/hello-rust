use std::{rc::Rc, borrow::BorrowMut};

pub struct SList {
  sentinel: IntNode,
  size: u64,
}
struct IntNode {
  pub item: i64,
  pub next: Option<Rc<IntNode>>
}

impl IntNode {
  fn new(item: i64, next: Option<Rc<IntNode>>) -> IntNode {
    IntNode{
      item,
      next
    }
  }
}

impl SList {

  fn new(item_op: Option<i64>) -> SList {
    match item_op {
      Some(item) => SList {
        sentinel: IntNode { item: i64::MAX, next: Some(Rc::new(IntNode{ item, next: None })) },
        size: 1
      },
      None => SList {
        sentinel: IntNode { item: i64::MAX, next: None },
        size: 0
      }
    }
  }

  fn add_first(&mut self, item: i64) {
    self.size += 1;
    self.sentinel.next = Some(Rc::new(IntNode {
      item,
      next: self.sentinel.next.take()
    }))
  }

  fn get_first(& self) ->  i64 {
    self.sentinel.next.as_ref().unwrap().item
  }

  // fn add_last(&mut self, item: i64) {
  //   self.size += 1;

 	// 	let mut p = &mut Rc::new(self.sentinel);

 	// 	/* Advance p to the end of the list. */
 	// 	while p.next.is_some() {
  //     let p = p.next.as_mut().unwrap();
 	// 		// p = p.next.unwrap().clone().borrow_mut();
 	// 	}

 	// 	p.next = Some(Rc::new(IntNode {
  //     item,
  //     next: None
  //   }));
  // }

  fn size(&self) -> u64 {
    self.size
  }

}

#[cfg(test)]
mod tests {
    use crate::s_list::SList;

  #[test]
  fn size_test() {
    let mut list = SList::new(None);
    list.add_first(42);
    assert_eq!(list.size(), 1);
  }
}