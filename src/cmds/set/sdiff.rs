use std::collections::HashSet;

use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Sdiff {
    keys: Vec<String>,
}

impl Sdiff {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();

        // 至少需要两个键（一个命令名，一个或多个集合键）
        if args.len() < 2 {
            return Err(Error::msg("ERR wrong number of arguments for 'sdiff' command"));
        }

        // 提取所有键
        let keys = args[1..].iter().map(|arg| arg.to_string()).collect();

        Ok(Sdiff { keys })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        if self.keys.is_empty() {
            return Err(Error::msg("ERR wrong number of arguments for 'sdiff' command"));
        }

        let mut iter = self.keys.iter();
        let first_key = iter.next().unwrap();

        // 获取第一个集合
        let first_set = match db.records.get(first_key) {
            Some(structure) => {
                match structure {
                    Structure::Set(set) => set.clone(),
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        return Ok(Frame::Error(f.to_string()));
                    }
                }
            },
            None => HashSet::new(), // 如果键不存在，视为空集
        };

        // 计算差集
        let mut difference: HashSet<String> = first_set;
        for key in iter {
            match db.records.get(key) {
                Some(structure) => {
                    match structure {
                        Structure::Set(set) => {
                            // 从差集中移除在当前集合中存在的元素
                            for member in set.iter() {
                                difference.remove(member);
                            }
                        },
                        _ => {
                            let f = "ERR Operation against a key holding the wrong kind of value";
                            return Ok(Frame::Error(f.to_string()));
                        }
                    }
                },
                None => {
                    // 如果键不存在，视为空集，不影响差集计算
                    continue;
                }
            }
        }

        // 将结果转换为 Frame::Array
        let members: Vec<Frame> = difference.into_iter()
            .map(|member| Frame::BulkString(member))
            .collect();

        Ok(Frame::Array(members))
    }
}