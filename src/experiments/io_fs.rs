// src/experiments/io_fs.rs

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

pub fn basic_file_ops() -> io::Result<()> {
    // 创建新文件
    let mut file = File::create("output.txt")?;
    writeln!(file, "Hello, Rust std!")?;

    // 读取文件内容
    let file = File::open("output.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("Line: {}", line?);
    }

    Ok(())
}

pub fn copy_file() -> io::Result<()> {
    fs::copy("output.txt", "copied_output.txt")?;
    println!("File copied!");
    Ok(())
}

pub fn list_current_dir() -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        println!("Found: {:?}", path);
    }
    Ok(())
}
