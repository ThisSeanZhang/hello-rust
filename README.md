# hello-rust
该项目是学习 Rust 过程中, 将书中每一章的例子复现

# 书本
* [the book](https://doc.rust-lang.org/book/)
* programming-rust

# 编写过程中, 发现的一些特性.
## 单链释放过程中容易出现栈溢出
这是因为在最后释放链对象时, 出现了递归释放. 如果栈特别长.就会导致栈溢出.

The problem that you are experiencing is because you have a giant linked-list of nodes. 
When that list is dropped, the first element tries to free all the members of the struct first. 
That means that the second element does the same, and so on, until the end of the list. 
This means that you will have a call stack that is proportional to the number of elements in your list!
> [stackoverflow](https://stackoverflow.com/questions/28660362/thread-main-has-overflowed-its-stack-when-constructing-a-large-tree)

## 可以自行定义多维数组
Rust 1.51的特性
```rust
// T could be any type
// W and H could be any usize value
pub struct Grid<T, const W: usize, const H: usize>
{
    array: [[T; W]; H],
}
impl<T, const W: usize, const H: usize> Default for Grid<T, W, H>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            array: [[T::default(); W]; H],
        }
    }
}
impl<T, const W: usize, const H: usize> Gridlike<T> for Grid<T, W, H> {
    fn width(&self) -> usize {
        W
    }

    fn height(&self) -> usize {
        H
    }

    fn get(&self, p: Point) -> &T {
        &self.array[p.y][p.x]
    }

    fn set_all_parallel<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(Point) -> T,
        T: Send,
    {
        use rayon::prelude::*;
        self.array.par_iter_mut().enumerate().for_each(|(y, row)| {
            for (x, item) in row.iter_mut().enumerate() {
                *item = setter(Point { x, y });
            }
        });
    }
}

```
> [Is it possible to control the size of an array using the type parameter of a generic?](https://stackoverflow.com/questions/28136739/is-it-possible-to-control-the-size-of-an-array-using-the-type-parameter-of-a-gen)
> [Rust 1.51 Stabilizes Const Generics MVP, Improves Cargo and Compile Times](https://www.infoq.com/news/2021/03/rust-1-51-released/)
> [Grids in Rust, part 2: const generics](https://blog.adamchalmers.com/grids-2/)

