[TOC]

# 官方文档





# 阶段一 跑一遍API

## 核心类型与语言元素

### Primitive Types

- [ ] `bool`
- [ ] `char`
- [ ] `i8`
- [ ] `i16`
- [ ] `i32`
- [ ] `i64`
- [ ] `i128`
- [ ] `isize`
- [ ] `u8`
- [ ] `u16`
- [ ] `u32`
- [ ] `u64`
- [ ] `u128`
- [ ] `usize`
- [ ] `f32`
- [ ] `f64`
- [ ] `array [T; N]`
- [ ] `slice [T]`
- [ ] `tuple (T, U, ...)`
- [ ] `str`
- [ ] `String`
- [ ] `fn`
- [ ] `unit ()`
- [ ] `never !`
- [ ] `reference (&T, &mut T)`
- [ ] `raw pointer (*const T, *mut T)`

###  Language Keywords

- [ ] `as`
- [ ] `async`
- [ ] `await`
- [ ] `break`
- [ ] `const`
- [ ] `continue`
- [ ] `crate`
- [ ] `dyn`
- [ ] `else`
- [ ] `enum`
- [ ] `extern`
- [ ] `false`
- [ ] `fn`
- [ ] `for`
- [ ] `if`
- [ ] `impl`
- [ ] `in`
- [ ] `let`
- [ ] `loop`
- [ ] `match`
- [ ] `mod`
- [ ] `move`
- [ ] `mut`
- [ ] `pub`
- [ ] `ref`
- [ ] `return`
- [ ] `self`
- [ ] `static`
- [ ] `struct`
- [ ] `super`
- [ ] `trait`
- [ ] `true`
- [ ] `type`
- [ ] `union`
- [ ] `unsafe`
- [ ] `use`
- [ ] `where`
- [ ] `while`





## 通用模块

####  std::option

- [ ] Option::map/filter/unwrap/unwrap_or_else
- [ ] Option::as_ref/as_mut
- [ ] Option::transpose

####  std::result

- [ ] Result::map/and_then
- [ ] Result::unwrap_or_else/expect
- [ ] Result::? operator

####  std::vec / std::slice

- [ ] Vec::new/with_capacity/push/pop
- [ ] Vec::retain/sort_by/split_off
- [ ] slice::split/split_at/chunks/windows
- [ ] iter()/iter_mut()/into_iter()

####  std::string / str

- [ ] String::from/push_str/replace
- [ ] str::parse/split/starts_with/contains

####  std::collections

- [ ] HashMap / HashSet 基础增删查改
- [ ] VecDeque::push_front/pop_back
- [ ] BinaryHeap / LinkedList 基本操作
- [ ] Entry API: or_insert, or_default

####  std::iter

- [ ] map/filter/collect
- [ ] chain/enumerate/flatten
- [ ] fuse/inspect/peekable

####  std::convert

- [ ] From/Into
- [ ] TryFrom/TryInto
- [ ] AsRef/AsMut

####  std::cmp / std::default

- [ ] PartialEq/PartialOrd/Eq/Ord
- [ ] Default::default()
- [ ] derive 对比手写 impl

###  std::fmt

- [ ] Display / Debug 实现
- [ ] format! / write! / writeln!
- [ ] Formatter trait 自定义格式化

### std::clone / std::marker

- [ ] Clone / Copy
- [ ] PhantomData
- [ ] Send / Sync / Unpin（了解 + 推断）



## 系统交互相关模块

### std::fs / std::io

- [ ] File::create/open/read_to_string/write_all
- [ ] BufReader / BufWriter 使用
- [ ] copy/rename/remove_file/metadata
- [ ] stdin / stdout / stderr

### std::path

- [ ] Path::new/join/extension
- [ ] PathBuf::from/to_str
- [ ] Path::exists/is_dir/is_file

### std::env

- [ ] env::args / args_os
- [ ] env::var / set_var / remove_var
- [ ] env::current_dir / set_current_dir
- [ ] env::temp_dir / home_dir

### std::net

- [ ] TcpListener::bind / accept
- [ ] TcpStream::connect / read / write
- [ ] UdpSocket::bind / send_to / recv_from
- [ ] SocketAddr / IpAddr 结构使用

### std::process

- [ ] Command::new/spawn/output/status
- [ ] 设置 stdin / stdout / stderr
- [ ] 读取进程输出 / 错误信息
- [ ] wait / kill 子进程控制

## std::os

- [ ] unix/windows 平台特性访问
- [ ] unix::fs::PermissionsExt 等

## std::time

- [ ] Instant::now / elapsed
- [ ] Duration::from_secs / as_millis
- [ ] thread::sleep(Duration)





## 并发与异步模块

### std::thread

- [ ] thread::spawn / join / sleep / yield_now
- [ ] thread::current / name / park / unpark

### std::sync

- [ ] Arc::clone / Arc + Mutex
- [ ] Mutex::lock / try_lock
- [ ] RwLock::read / write
- [ ] Condvar::wait / notify_one / notify_all

### std::sync::mpsc

- [ ] channel / send / recv / try_recv
- [ ] multi-producer single-consumer 模式测试

### std::sync::atomic

- [ ] AtomicBool / AtomicUsize
- [ ] fetch_add / fetch_sub / store / load
- [ ] Ordering::SeqCst / Relaxed / Acquire / Release

### std::cell / std::rc

- [ ] RefCell::borrow / borrow_mut
- [ ] Rc::clone / Rc::strong_count / Rc::downgrade
- [ ] Rc<RefCell<T>> 常见组合使用

### std::future / std::task

- [ ] 实现一个自定义 Future
- [ ] 使用 Context / Waker 模拟异步调度
- [ ] Poll::Ready / Pending 模拟器

### std::pin

- [ ] Pin<Box<T>> 的创建与使用
- [ ] Unpin trait 推导与约束
- [ ] self-referential struct 示例



## 辅助模块 & 宏 & 实验性

### 宏（macros）

- [ ] println! / eprintln! / print! / dbg!
- [ ] assert! / assert_eq! / assert_ne! / debug_assert!
- [ ] panic! / todo! / unimplemented! / unreachable!
- [ ] format! / write! / writeln! / format_args!
- [ ] vec! / matches! / include_str! / include_bytes!
- [ ] cfg! / cfg_attr! / env! / option_env!

### std::panic / std::backtrace / std::hint

- [ ] panic::catch_unwind
- [ ] Backtrace::capture
- [ ] hint::black_box / unreachable_unchecked

### std::ffi

- [ ] CString::new / as_ptr
- [ ] CStr::from_ptr / to_str

### std::mem / std::ptr

- [ ] mem::size_of / transmute / replace / swap
- [ ] ptr::read / write / copy / null / offset

### std::alloc

- [ ] GlobalAlloc trait
- [ ] alloc / dealloc / realloc 函数
- [ ] Layout::new / from_size_align

### std::marker

- [ ] PhantomData<T>
- [ ] Send / Sync / Unpin / 'static 推导实验

### std::primitive / std::prelude

- [ ] 了解 prelude 默认导入 trait
- [ ] primitive 模块中导出的核心类型

### 实验性模块（仅了解）

- [ ] assert_matches
- [ ] async_iter
- [ ] bstr / autodiff / simd
- [ ] rangeExperimental / intrinsics / unsafe_binder