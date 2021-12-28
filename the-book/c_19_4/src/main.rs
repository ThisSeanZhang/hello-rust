use std::io::Error;
use std::{fmt, io};
use std::fmt::Arguments;

fn main() {
    /// 使用newtype模式实现类型安全与抽象

    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --略--
    }
    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --略--
        Box::new(|| ())
    }

    // 减少冗长的类型标注
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type2(f: Thunk) {
        // --略--
    }
    fn returns_long_type2() -> Thunk {
        // --略--
        Box::new(|| ())
    }

    // 另外的例子
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
        fn flush(&mut self) -> Result<(), Error>;
        fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    }
    type MyResult<T> = Result<T, Error>;
    pub trait Write2 {
        fn write(&mut self, buf: &[u8]) -> MyResult<usize>;
        fn flush(&mut self) -> MyResult<()>;
        fn write_all(&mut self, buf: &[u8]) -> MyResult<()>;
        fn write_fmt(&mut self, fmt: Arguments) -> MyResult<()>;
    }

    /// 永不返回的Never类型

    fn bar() -> ! {
        // --略--
        println!("ccccc")
    }

    // loop {
    //     io::stdin().read_line(&mut guess) .expect("Failed to read line");
    //     let guess: u32 = match guess.trim().parse() { // 所以此处能进行推断是数字类型
    //         Ok(num) => num,
    //         // Err(_) => continue, // 因为此处返回的类型是Never
    //         Err(_) => break, // 因为此处返回的类型是Never
    //     };
    // }

    /// 动态大小类型和Size trait
    //动 态 大 小 类 型
    // （Dynamically Sized Type，DST）的概念，它有时也被称作不确
    // 定大小类型（unsized type），这些类型使我们可以在编写代码时使
    // 用只有在运行时才能确定大小的值
    //让我们来深入研究一个叫作str的动态大小类型，这个类型几乎贯
    // 穿了本书的所有章节。没错，我们会在这里讨论str本身而不是&str，
    // str正好是一个动态大小类型。我们只有在运行时才能确定字符串的长
    // 度，这也意味着我们无法创建一个str类型的变量，或者使用str类型
    // 来作为函数的参数。如下所示的代码无法正常工作：
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
    // Rust需要在编译时确定某个特定类型的值究竟会占据多少内存，
    // 而同一类型的所有值都必须使用等量的内存。假如Rust允许我们写出
    // 上面这样的代码，那么这两个str的值就必须要占据等量的空间。但它
    // 们确实具有不同的长度：s1需要12字节的存储空间，而s2则需要15
    // 字节。这也是我们无法创建出动态大小类型变量的原因。
    // 那么我们应该怎么处理类似的需求呢？你应该已经非常熟悉本例
    // 中出现的情形了：我们会把s1与s2的类型从str修改为&str。回忆一
    // 下第4章的“字符串切片”一节，我们当时指出，切片的数据结构中会
    // 存储数据的起始位置及切片的长度。
    // 因此，尽管&T被视作存储了T所在内存地址的单个值，但&str实
    // 际上是由两个值组成的：str的地址与它的长度。这也使我们可以在编
    // 译时确定&str值的大小：其长度为usize长度的两倍。换句话说，无
    // 论&str指向了什么样的字符串，我们总是能够知道&str的大小。这就
    // 是Rust中使用动态大小类型的通用方式：它们会附带一些额外的元数
    // 据来存储动态信息的大小。我们在使用动态大小类型时总是会把它的
    // 值放在某种指针的后面。
    // 我们可以将str与所有种类的指针组合起来，例如Box<str>或
    // Rc<str>等。事实上，你在之前的章节就已经见到过类似的用法了，
    // 只不过当时使用了另外一种动态大小类型：trait。每一个trait都是一
    // 个可以通过其名称来进行引用的动态大小类型。在第17章的“使用
    // trait对象来存储不同类型的值”一节中曾经提到过，为了将trait用作
    // trait对象，我们必须将它放置在某种指针之后，比如&dyn Trait或
    // Box<dyn Trait>（Rc<dyn Trait>也可以）之后。
    // 为了处理动态大小类型，Rust还提供了一个特殊的Sized trait来
    // 确定一个类型的大小在编译时是否可知。编译时可计算出大小的类型
    // 会自动实现这一trait。另外，Rust还会为每一个泛型函数隐式地添加
    // Sized约束。也就是说，下面定义的泛型函数：
    // fn generic<T>(t: T) {
    //  // --略--
    // }
    // 实际上会被隐式地转换为：
    // fn generic<T: Sized>(t: T) {
    //  // --略--
    // }
    // 在默认情况下，泛型函数只能被用于在编译时已经知道大小的类
    // 型。但是，你可以通过如下所示的特殊语法来解除这一限制：
    // fn generic<T: ?Sized>(t: &T) {
    //  // --略--
    // }
    // ?Sized trait约束表达了与Sized相反的含义，我们可以将它读
    // 作“T可能是也可能不是Sized的”。这个语法只能被用在Sized上，而
    // 不能被用于其他trait。
    // 另外还需要注意的是，我们将t参数的类型由T修改为了&T。因为
    // 类型可能不是Sized的，所以我们需要将它放置在某种指针的后面。
    // 在本例中，我们选择使用引用。



}
