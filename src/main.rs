use std::{env, process};
use minigrep::Args;

fn main() {
//    let args: Vec<String> = env::args().collect();
    let args = Args::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error：{}", err);
        process::exit(1);
    });

    if let Err(_) = minigrep::run(args) {
        eprintln!("参数解析错误");
        process::exit(1);
    }
}

