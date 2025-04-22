mod experiments;


fn main() {
    println!("🧪 std::fs 实验开始");

    if let Err(e) = experiments::io_fs::basic_file_ops() {
        eprintln!("❌ 文件操作失败: {}", e);
    }

    experiments::io_fs::copy_file().unwrap();
    experiments::io_fs::list_current_dir().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
            let s1 = String::from("hello");
            let s2 = s1;          // ❌ s1 被 move
            // println!("{}", s1);   // ❌ 编译错误：s1 已被移动
    }
}