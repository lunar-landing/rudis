use std::collections::BTreeMap;

use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Zincrby {
    key: String,
    increment: f64,
    member: String,
}

impl Zincrby {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() != 4 {
            return Err(Error::msg("ERR wrong number of arguments for 'zincrby' command"));
        }
        
        let key = args[1].to_string(); // 键
        let increment = args[2].parse::<f64>().map_err(|_| Error::msg("ERR value is not a valid float"))?;
        let member = args[3].to_string(); // 成员
        
        Ok(Zincrby { key, increment, member })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // 获取当前成员的分数，如果不存在则默认为0.0
        let current_score = match db.records.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::SortedSet(set) => {
                        // 如果成员已存在，获取其当前分数，否则为0.0
                        set.entry(self.member.clone()).or_insert(0.0);
                        *set.get(&self.member).unwrap()
                    },
                    _ => {
                        // 键存在但不是有序集合类型
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        return Ok(Frame::Error(f.to_string()));
                    }
                }
            },
            None => {
                // 键不存在，创建新的有序集合
                let mut set = BTreeMap::new();
                set.insert(self.member.clone(), 0.0);
                db.records.insert(self.key.clone(), Structure::SortedSet(set));
                0.0
            }
        };

        // 计算新分数
        let new_score = current_score + self.increment;

        // 更新分数
        match db.records.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::SortedSet(set) => {
                        set.insert(self.member.clone(), new_score);
                    },
                    _ => {} // 这种情况已经在上面处理过了
                }
            },
            None => {} // 这种情况已经在上面处理过了
        }

        // 返回新分数
        Ok(Frame::BulkString(new_score.to_string()))
    }
}