use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use regex::Regex;

// 使用 Mutex 保护的 HashMap 来缓存编译后的正则表达式
fn regex_cache() -> &'static Mutex<HashMap<String, Regex>> {
    static CACHE: OnceLock<Mutex<HashMap<String, Regex>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn is_match(key: &str, pattern: &str) -> bool {
    fn convert_pattern(pattern: &str) -> String {
        let mut regex_pattern = String::new();
        let mut chars = pattern.chars().peekable();
        while let Some(p) = chars.next() {
            match p {
                '*' => regex_pattern.push_str(".*"), 
                '?' => regex_pattern.push('.'),    
                '[' => {
                    regex_pattern.push('[');
                    if let Some(next) = chars.peek() {
                        if *next == '^' {
                            regex_pattern.push('^');
                            chars.next(); // 跳过 '^'
                        }
                    }
                    while let Some(ch) = chars.next() {
                        if ch == ']' {
                            break;
                        }
                        regex_pattern.push(ch);
                    }
                    regex_pattern.push(']');
                }
                _ => regex_pattern.push(p)
            }
        }
        regex_pattern
    }
    
    // 尝试从缓存中获取已编译的正则表达式
    let regex = {
        let cache = regex_cache().lock().unwrap();
        if let Some(regex) = cache.get(pattern) {
            regex.clone()
        } else {
            drop(cache); // 释放读锁后再进行写操作
            let regex_pattern = convert_pattern(pattern);
            let regex = Regex::new(&regex_pattern).unwrap();
            let mut cache = regex_cache().lock().unwrap();
            cache.insert(pattern.to_string(), regex.clone());
            regex
        }
    };
    
    regex.is_match(key)
}