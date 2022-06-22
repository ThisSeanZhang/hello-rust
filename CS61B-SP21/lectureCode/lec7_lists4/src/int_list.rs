// struct ItemArray<const COUNT:usize> {
//   arr: [i64; COUNT],
// }
// impl<const COUNT: usize> Default for ItemArray<COUNT>
// where
// {
//     fn default() -> Self {
//         Self {
//           arr: [0; COUNT],
//         }
//     }
// }

struct AList {
  // TODO temporary use Vec
  items: Vec<i64>,
  size: usize
}

impl AList {
  fn new() -> Self {
    Self{
      items: Vec::with_capacity(100),
      size: 0
    }
  }

  fn resize(&mut self, size: usize) {
    // let a: ItemArray<size> = ItemArray::default();
    self.items.resize_with(size, Default::default);

    // self.items = 
  }
}