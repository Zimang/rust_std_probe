> Second, implicit methods on [primitive types](https://doc.rust-lang.org/book/ch03-02-data-types.html) are documented here. This can be a source of confusion for two reasons:
>
> 1. While primitives are implemented by the compiler, the standard library implements methods directly on the primitive types (and it is the only library that does so), which are [documented in the section on primitives](https://doc.rust-lang.org/std/index.html#primitives).
> 2. The standard library exports many modules *with the same name as primitive types*. These define additional items related to the primitive type, but not the all-important methods.

这句话的意思是

原始数据类型是由编译器定义，但是一些方法并非编译器实现，此外它们的文档（重点是文档）分散为两个部分，std模块这里可能只含有一些少量或者极少用到的方法，常用的则需要到 [documented in the section on primitives](https://doc.rust-lang.org/std/index.html#primitives)去看

想让原始数据类型也用上方法，他们的实现一般是由核心库实现的`core`



> Note the documentation for the primitives [`str`](https://doc.rust-lang.org/std/primitive.str.html) and [`[T\]`](https://doc.rust-lang.org/std/primitive.slice.html) (also called ‘slice’). Many method calls on [`String`](https://doc.rust-lang.org/std/string/struct.String.html) and [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) are actually calls to methods on [`str`](https://doc.rust-lang.org/std/primitive.str.html) and [`[T\]`](https://doc.rust-lang.org/std/primitive.slice.html) respectively, via [deref coercions](https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods).

一些方法通过解引用复用了其他的代码，这意味`String`的方法，你可能需要去`str`去看；`Vec<T>`的则需要去`[T]`上去看



> Third, the standard library defines [The Rust Prelude](https://doc.rust-lang.org/std/prelude/index.html), a small collection of items - mostly traits - that are imported into every module of every crate. The traits in the prelude are pervasive, making the prelude documentation a good entry point to learning about the library.

`std`定义了一个叫做`prelude`的集合(主要包含`trait`). 这些`trait`被广泛应用，基本上就是常用功能，是一个学习的切入点。`prelude`默认导入，所有`crate`都可以用



> And finally, the standard library exports a number of standard macros, and [lists them on this page](https://doc.rust-lang.org/std/index.html#macros) (technically, not all of the standard macros are defined by the standard library - some are defined by the compiler - but they are documented here the same). Like the prelude, the standard macros are imported by default into all crates.

标准库导出了一系列的标准宏（并非所有标准宏都由标准库定义）,这些标准宏也是默认导入的



容器与集合

> The [`option`](https://doc.rust-lang.org/std/option/index.html) and [`result`](https://doc.rust-lang.org/std/result/index.html) modules define optional and error-handling types, [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html) and [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html). The [`iter`](https://doc.rust-lang.org/std/iter/index.html) module defines Rust’s iterator trait, [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html), which works with the [`for`](https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for) loop to access collections.

- `option,result`:定义`Option` 和`Result`,处理错误与可选类型
- `iter:`定义`Iterator`用于`for`遍历access 集合



> The standard library exposes three common ways to deal with contiguous regions of memory:
>
> - [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) - A heap-allocated *vector* that is resizable at runtime.
> - [`[T; N]`](https://doc.rust-lang.org/std/primitive.array.html) - An inline *array* with a fixed size at compile time.
> - [`[T]`](https://doc.rust-lang.org/std/primitive.slice.html) - A dynamically sized *slice* into any other kind of contiguous storage, whether heap-allocated or not.

rust提供三种方式处理内存连续的数据结构

- `Vec`:堆分配的可变长动态数组(runtime)
- `[T;N]`:  嵌在栈上的定长数组
- `[T]`: a slice into ？这里的`into`意思是将一段连续的内存（无论是栈还是堆）切为`slice`
  - 这里要指出的是`slice`不关心连续内存是在堆，栈，它都可以安全的表示为切片的形式



> Slices can only be handled through some kind of *pointer*, and as such come in many flavors such as:
>
> - `&[T]` - *shared slice*
> - `&mut [T]` - *mutable slice*
> - [`Box<[T]>`](https://doc.rust-lang.org/std/boxed/index.html) - *owned slice*

slice只能通过以下指针的形式处理

- `&[T]`: 共享只读切片
- `&mut [T]`可变切片
- `Box<[T]>`: 拥有所有权的切片，在堆上



> [`str`](https://doc.rust-lang.org/std/primitive.str.html), a UTF-8 string slice, is a primitive type, and the standard library defines many methods for it. Rust [`str`](https://doc.rust-lang.org/std/primitive.str.html)s are typically accessed as immutable references: `&str`. Use the owned [`String`](https://doc.rust-lang.org/std/string/struct.String.html) for building and mutating strings.

这里比较了`str`和`String`

- `str`：原始类型，通常以不可变引用`&str`的形式被访问，因为绝大部分使用情况下字符串是只读的
- `String`: 构建或者`mutate`字符串用,这里`mutate`的意思是修改原对象本身，不是创建新的对象或者替换原对象



> For converting to strings use the [`format!`](https://doc.rust-lang.org/std/macro.format.html) macro, and for converting from strings use the [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) trait.

想把任何东西“格式化为字符串”？用 `format!`。

 想把字符串“解析成某种类型”？用 `.parse()` 或 `FromStr`。 //`parse`是gpt给出的，我也不太懂



> Data may be shared by placing it in a reference-counted box or the [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) type, and if further contained in a [`Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html) or [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html), may be mutated as well as shared. Likewise, in a concurrent setting it is common to pair an atomically-reference-counted box, [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html), with a [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) to get the same effect.

- `or the`是考验翻译的地方，实际上这里讲的就是`Rc`
- 你可以用`Rc`来共享一个数据值，想要既共享又可变，就要用`Cell`或者`RefCell`来做容器了
- 在多线程环境下，我们需要用`Arc`搭配`Mutex`来达到相同的效果



> The `collections` module defines maps, sets, linked lists and other typical collection types, including the common `HashMap<K, V>`.

- `collections`模块还提供很多其他集合类型，包括最常用的`HashMap<K,V>`





> Besides basic data types, the standard library is largely concerned with abstracting over differences in common platforms, most notably Windows and Unix derivatives.

除了原始类型之外，标准库的另一个关注点在于屏蔽不同平台的系统差异性



> Many parts of the standard library are expected to work before and after `main()`; but this is not guaranteed or ensured by tests. It is recommended that you write your own tests and run them on each platform you wish to support. This means that use of `std` before/after main, especially of features that interact with the OS or global state, is exempted from stability and portability guarantees and instead only provided on a best-effort basis. Nevertheless bug reports are appreciated.

- main会在调用前后做很多事情，绝非一句程序入口点就可以说明
  - 初始化标准输出/错误
  - 设置线程本地变量
  - 注册panic钩子
  - 建立通道任务调度等机制
- 标准库的一些部分工作于main之外，这些部分的功能绝无任何功能上的保证，仅仅是以最大努力作为原则



> On the other hand `core` and `alloc` are most likely to work in such environments with the caveat that any hookable behavior such as panics, oom handling or allocators will also depend on the compatibility of the hooks.

- `core`和`alloc`在这种情况下是大多是可以工作的，但是需要注意的是，但凡可钩住的行为都依赖于钩子的兼容性
  - hookabel behavior 允许自定义一些运行时行为
    - panci处理
    - OOM行为
    - 全局分配器

> 
>
> Some features may also behave differently outside main, e.g. stdio could become unbuffered, some panics might turn into aborts, backtraces might not get symbolicated or similar.

一些标准库的功能在main之外会变得异常

- 输出不会被缓冲，可能直接打印甚至不打印
- panic变为abort
- backstrace无法符号化，显示`<unknow>`



> Non-exhaustive list of known limitations:
>
> - after-main use of thread-locals, which also affects additional features:
>   - [`thread::current()`](https://doc.rust-lang.org/std/thread/fn.current.html)
> - before-main stdio file descriptors are not guaranteed to be open on unix platforms

未穷尽的已知缺陷包括

1. after-main：线程局部变量（thread-local）不可靠
2. before-main：Unix 上 stdio 文件描述符不保证打开





#### Prelude

rust的标准库非常丰富，如果每次用都要显式导入那么代码就会变得臃肿，反之把所有内容预导入也不是一件好事，所以我们就倾向于预导入一些常用的基本类型和trait，旨在减少冗余 import，同时避免污染命名空间。它保持最小规模，只包含几乎每个程序都会用到的东西，特别是 trait，因为它们是方法语法的基础。



> ## Other preludes
>
> Preludes can be seen as a pattern to make using multiple types more convenient. As such, you’ll find other preludes in the standard library, such as [`std::io::prelude`](https://doc.rust-lang.org/std/io/prelude/index.html). Various libraries in the Rust ecosystem may also define their own preludes.
>
> The difference between ‘the prelude’ and these other preludes is that they are not automatically `use`’d, and must be imported manually. This is still easier than importing all of their constituent components.

Rust 里的 “prelude” 不止一个。虽然只有一个是自动导入的（“the prelude”），但你在 `std` 和第三方库中也会发现很多类似 `::prelude` 命名的模块。

“prelude” 本质上是一种设计模式 —— 把常用的类型、trait 等集中放进一个模块，让你只要 `use xxx::prelude::*;`，就能引入它们，而不是每个都 `use` 一次。

比如

```rust
use std::io::prelude::*;
//这行代码会帮你引入一堆常用 trait，例如：
//Read,Write,BufRead,Seek
```

其他的一些库也会定义他们的`Prelude`,不同之处在于，这里的`prelude`自动导入，其他的则需要手动导入



Rust 在 每个 edition 中维护自己的 prelude 列表，解决向后兼容与新特性引入的矛盾。

第一个`Prelude`是`std::prelude::v1`用于`rust 2015`和`rust 2018`

```rust
/// The 2015 version of the prelude of The Rust Standard Library. 
#[stable(feature = "prelude_2015", since = "1.55.0")]
pub mod rust_2015 {
    #[stable(feature = "prelude_2015", since = "1.55.0")]
    #[doc(no_inline)]
    pub use super::v1::*;
} 
/// The 2018 version of the prelude of The Rust Standard Library. 
#[stable(feature = "prelude_2018", since = "1.55.0")]
pub mod rust_2018 {
    #[stable(feature = "prelude_2018", since = "1.55.0")]
    #[doc(no_inline)]
    pub use super::v1::*;
}
```

包括

- `std::marker::{Copy, Send, Sized, Sync, Unpin}`, marker traits that indicate fundamental properties of types.
- `std::ops::{Fn, FnMut, FnOnce}`, and their analogous async traits, `std::ops::{AsyncFn, AsyncFnMut, AsyncFnOnce}`.
- `std::ops::Drop`, for implementing destructors.
- `std::mem::drop`, a convenience function for explicitly dropping a value.
- `std::mem::{size_of, size_of_val}`, to get the size of a type or value.
- `std::mem::{align_of, align_of_val}`, to get the alignment of a type or value.
- `std::boxed::Box`, a way to allocate values on the heap.
- `std::borrow::ToOwned`, the conversion trait that defines [`to_owned`](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html#tymethod.to_owned), the generic method for creating an owned type from a borrowed type.
- `std::clone::Clone`, the ubiquitous trait that defines [`clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html#tymethod.clone), the method for producing a copy of a value.
- `std::cmp::{PartialEq, PartialOrd, Eq, Ord}`, the comparison traits, which implement the comparison operators and are often seen in trait bounds.
- `std::convert::{AsRef, AsMut, Into, From}`, generic conversions, used by savvy API authors to create overloaded methods.
- `std::default::Default`, types that have default values.
- `std::iter::{Iterator, Extend, IntoIterator, DoubleEndedIterator, ExactSizeIterator}`, iterators of various kinds.
- `std::option::Option::{self, Some, None}`, a type which expresses the presence or absence of a value. This type is so commonly used, its variants are also exported.
- `std::result::Result::{self, Ok, Err}`, a type for functions that may succeed or fail. Like [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html), its variants are exported as well.
- `std::string::{String, ToString}`, heap-allocated strings.
- `std::vec::Vec`, a growable, heap-allocated vector.



第二个则是`prelude_2021`在v1的基础上加上`core::prelude::rust_2021::*;`它加上了

- `std::convert::{TryFrom, TryInto}`.
- `std::iter::FromIterator`.



第二个则是`prelude_2024`在v1的基础上加上`core::prelude::rust_2024::*;` 它包含`core::prelude::rust_2021::*;`并加上了：

- `std::future::{Future, IntoFuture}`.