struct IntList {
  pub first:  i64,
  pub rest: Option<Box<IntList>>,
}

impl IntList {
  pub fn new(num: i64, next: Option<IntList>) -> IntList {
    let rest = match next {
      None => None,
      Some(n) => Some(Box::new(n))
    };
    IntList { first: num, rest }
  }

  /** Return the size of the list using... recursion! */
  pub fn size(&self) -> u64 {
    if let Some(rest_rc) = &self.rest {
        return 1 + rest_rc.size();
    }
    return 1;
  }

  fn iterative_size(&self) -> u64 {
		let mut total_size = 1;
    let mut p = self.rest.as_ref();
		while p.is_some() {
			total_size += 1;
      p = p.unwrap().rest.as_ref();
		}
		return total_size;
	}

  pub fn get(&self, index: u64) -> Option<i64> {
    if index == 0 {
      return Some(self.first);
    }

    if let Some(rest_rc) = &self.rest {
      return rest_rc.get(index - 1);
    }

    None
  }
}

#[cfg(test)]
mod test {
    use crate::int_list::IntList;

  #[test]
  fn check_size() {
    let a = IntList::new(3, None);
    let b = IntList::new(2, Some(a));
    let c = IntList::new(1, Some(b));
    assert_eq!(c.size(), 3);
    assert_eq!(c.iterative_size(), c.size())
  }

  #[test]
  fn get_index() {
    let a = IntList::new(3, None);
    let b = IntList::new(2, Some(a));
    let c = IntList::new(1, Some(b));
    assert_eq!(c.get(4), None);
    assert_eq!(c.get(2).unwrap(), 3);
  }
}
