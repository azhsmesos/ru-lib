
mod time_util;

use std::env;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::time_util::{conversion};

fn main() {
    // 初始化工具
    init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        tracing::error!("请重新输入，当前无参数");
        return;
    }

    let option = &args[1];
    let mut vec: Vec<String> = args.clone();
    match option.as_str() {
        "time" => {
            let param = vec.split_off(2)
                .iter()
                .map(|s| s as &str)
                .collect::<Vec<&str>>().join(" ");
            conversion(&param);
        }
        _ => {
            tracing::warn!("无效的命令")
        }
    }
}

fn init() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .init()
}
