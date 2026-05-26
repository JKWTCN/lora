use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // 获取当前 UTC 日期
    let build_date = chrono::Utc::now().format("%Y-%m-%d").to_string();

    // 生成包含编译日期的 Rust 代码
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_info.rs");
    let mut f = File::create(&dest_path).unwrap();
    writeln!(f, "pub const BUILD_DATE: &str = \"{}\";", build_date).unwrap();

    // 重新编译时重新生成
    println!("cargo:rerun-if-changed=build.rs");

    // 运行 tauri_build
    tauri_build::build();
}
