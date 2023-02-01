use minigrep::Config;
use std::{env, process};

/**
 * 关注分离
 *
 * main函数的责任：
 * - 使用参数值调用命令行解析逻辑
 * - 设置任何其它的配置
 * - 调用lib.rs中的run函数
 * - 如果run函数返回错误，则处理这个错误
 *
 * lib.rs处理任务逻辑
 *
 */
fn main() {
    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args).unwrap_or_else(|err| {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("参数解析问题：{}", err);
        process::exit(1);
    });

    println!("搜索关键字: {}", config.query);
    println!("文件: {}", config.filename);
    if let Err(e) = minigrep::run(config) {
        eprintln!("程序错误：{}", e)
    }
}
