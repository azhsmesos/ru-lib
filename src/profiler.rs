use std::fs;
use std::process::Command;
pub fn execute_profiler(param: &str) {
    println!("【executing】");
    let out_path = format!("/Users/zhenhangzhao/Documents/profiler/{}.html", param);
    if fs::metadata(out_path.clone()).is_ok() {
        if let Err(err) = fs::remove_file(out_path.as_str()) {
            tracing::error!("failed to delete file: {}, err: {}", out_path, err);
        };
    }
    let output = Command::new("/Users/zhenhangzhao/Downloads/async-profiler-2.9-macos/profiler.sh")
        .arg("-d")
        .arg("30")
        .arg("-f")
        .arg(out_path)
        .arg(param)
        .output();
    match output {
        Ok(_) => tracing::info!("【profiler execute successfully!!!】pid={}", param),
        Err(_) => tracing::error!("【profiler execute failed】pid={}, output={:?}", param, output),
    }
}