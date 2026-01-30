use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Zrange {
    key: String,
    start: i64,
    stop: i64,
    with_scores: bool,
}

impl Zrange {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() < 4 {
            return Err(Error::msg("ERR wrong number of arguments for 'zrange' command"));
        }
        
        let key = args[1].to_string();
        let start = args[2].to_string().parse::<i64>().map_err(|_| Error::msg("ERR value is not an integer or out of range"))?;
        let stop = args[3].to_string().parse::<i64>().map_err(|_| Error::msg("ERR value is not an integer or out of range"))?;
        
        // 检查是否有WITHSCORES参数
        let mut with_scores = false;
        for arg in args.iter().skip(4) {
            if arg.to_uppercase() == "WITHSCORES" {
                with_scores = true;
                break;
            }
        }
        
        Ok(Zrange { key, start, stop, with_scores })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.records.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::SortedSet(set) => {
                        // 跳表已经按 (score, member) 排序，直接使用
                        let len = set.len() as i64;
                        
                        // 处理负数索引
                        let start_idx = if self.start < 0 {
                            (len + self.start).max(0)
                        } else {
                            self.start.min(len)
                        };
                        
                        let stop_idx = if self.stop < 0 {
                            (len + self.stop).max(-1)
                        } else {
                            self.stop.min(len - 1)
                        };
                        
                        // 确保索引有效
                        if start_idx > stop_idx || start_idx >= len {
                            return Ok(Frame::Array(vec![]));
                        }
                        
                        // 使用跳表的 range 方法，O(log n + m) 时间复杂度
                        let start_idx = start_idx as usize;
                        let stop_idx = stop_idx as usize;
                        let selected_members = set.range(start_idx, stop_idx);
                        
                        // 构建返回结果
                        let mut result = Vec::new();
                        for (member, score) in selected_members {
                            result.push(Frame::BulkString(member));
                            if self.with_scores {
                                result.push(Frame::BulkString(score.to_string()));
                            }
                        }
                        
                        Ok(Frame::Array(result))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                // 键不存在，返回空数组
                Ok(Frame::Array(vec![]))
            }
        }
    }
}