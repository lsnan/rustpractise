use std::env; 
use std::process;
use minigrep::Config;

fn main() {
    let args :Vec<String> = env::args().collect();   // 无法接收哥法字符
    // env::args_os() // OsString 类型, 可以接收非法字符, 为了简单,不用这个了

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
