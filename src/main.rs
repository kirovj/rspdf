use std::{env, fs, process::Command, time::Instant};

fn separate(exe_path: &str, name: String) {
    let command = format!("{} {}.pdf {}\\{}-%d.pdf", exe_path, name, name, name);
    Command::new("cmd")
        .arg("/c")
        .arg(command.as_str())
        .status()
        .expect("failed to execute process");
}

fn main() {
    println!(
        "Pdf 文件切割工具 v{} by wuyiting@myhexin.com",
        env!("CARGO_PKG_VERSION")
    );
    println!("功能：将当前目录下的所有 pdf 文件按页切分\n=======================================");
    let current_path = env::current_dir().expect("get current dir failed");
    let current_dir = String::from(current_path.to_str().unwrap());
    let pdf_separate_path = format!("{}\\library\\bin\\pdfseparate.exe", current_dir);
    println!("正在查找当前目录中的 pdf 文件...");
    for entry in fs::read_dir(current_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == "pdf" {
                if let Some(filename) = path.file_name() {
                    println!("开始切割: {:?}", filename);
                    let start_time = Instant::now();
                    let name = String::from(filename.to_str().unwrap()).replace(".pdf", "");
                    let _ = fs::create_dir(format!("{}\\{}", current_dir, name));
                    separate(&pdf_separate_path, name);
                    println!("切割完成，用时 {}s", start_time.elapsed().as_secs());
                }
            }
        }
    }
}
