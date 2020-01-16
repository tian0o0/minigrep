use std::{fs, env};
use std::error::Error;

pub struct Args {
    query: String,
    filename: String,
    sensitive: bool,
}

impl Args {
    pub fn new(mut args: env::Args) -> Result<Args, &'static str> {
//        if args.len() < 3 {
//            return Err("缺少参数");
//        }
//        let query = args[1].clone();
//        let filename = args[2].clone();
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("没有query"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("没有文件名"),
        };
        let sensitive = env::var("$GREP_SENSITIVE").is_err();
        Ok(Args { query, filename, sensitive })
    }
}
//Box<dyn Error> 被称为 “trait 对象”（“trait object”），第十七章 “为使用不同类型的值而设计的 trait 对象” 部分会做介绍。
// 目前可以理解 Box<dyn Error> 为使用 ? 时 main 允许返回的 “任何类型的错误”
pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.filename)?;
    let results = if args.sensitive {
        search(&args.query, &contents)
    }else {
        search_case_insensitive(&args.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//    let mut res = Vec::new();
//    for line in contents.lines() {
//        if line.contains(query) {
//            res.push(line);
//        }
//    }
//    res
    contents.lines().filter(|line| line.contains(query)).collect()
}
// 忽略大小写
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}