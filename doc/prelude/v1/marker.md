什么是marker trait

1. marker trait 是 没有任何方法 的 trait
2. 它们的作用是用来给类型“贴标签”或“声明属性”

3. 通过这些 trait，Rust 编译器能根据类型的能力作出行为判断（如自动实现、线程安全、是否可拷贝等）

[std::marker](https://doc.rust-lang.org/std/marker/index.html)::{[Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html), [Send](https://doc.rust-lang.org/std/marker/trait.Send.html), [Sized](https://doc.rust-lang.org/std/marker/trait.Sized.html), [Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html), [Unpin](https://doc.rust-lang.org/std/marker/trait.Unpin.html)}



Copy

`Copy` 是一个 marker trait，表示类型的值可以被按位复制（bitwise copy），无需 move、无需手动 clone。

rust中变量赋值的三种语义

| 行为                  | 描述                     | 举例                      |
| --------------------- | ------------------------ | ------------------------- |
| **move（移动）**      | 所有权转移，原变量失效   | `let s2 = s1;` (`String`) |
| **clone（显式复制）** | 调用 `.clone()` 方法     | `let s2 = s1.clone();`    |
| **copy（隐式复制）**  | 变量赋值时直接复制位内容 | `let x2 = x1;` (`i32`)    |

`Copy` 是 Rust 编译器为某些类型提供的一种“优化的 move 替代”。当你进行赋值（`let b = a;`）或者函数传参（`foo(a)`）时，Rust 不会 move `a`，而是直接做一份按位拷贝（bitwise copy），`a` 和 `b` 都可以继续使用。