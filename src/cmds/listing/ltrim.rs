use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Ltrim {
    key: String,
    start: i64,
    stop: i64,
}

impl Ltrim {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let key = frame.get_arg(1);
        let start = frame.get_arg(2);
        let stop = frame.get_arg(3);

        if key.is_none() || start.is_none() || stop.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'ltrim' command"));
        }

        let final_key = key.unwrap().to_string();

        let start = match start.unwrap().parse::<i64>() {
            Ok(n) => n,
            Err(_) => return Err(Error::msg("ERR value is not an integer or out of range")),
        };

        let stop = match stop.unwrap().parse::<i64>() {
            Ok(n) => n,
            Err(_) => return Err(Error::msg("ERR value is not an integer or out of range")),
        };

        Ok(Ltrim {
            key: final_key,
            start,
            stop,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::List(list) => {
                        let len = list.len() as i64;
                        
                        // 处理负数索引
                        let start = if self.start < 0 { 
                            std::cmp::max(0, len + self.start) 
                        } else { 
                            self.start 
                        };
                        
                        let stop = if self.stop < 0 { 
                            len + self.stop 
                        } else { 
                            self.stop 
                        };

                        // 确保索引在有效范围内
                        let start = std::cmp::max(0, start);
                        let stop = std::cmp::min(len - 1, stop);

                        // 如果范围无效，则清空列表
                        if start > stop || start >= len || stop < 0 {
                            list.clear();
                        } else {
                            // 保留指定范围内的元素
                            // 先移除范围之后的元素
                            if stop + 1 < len {
                                list.drain((stop + 1) as usize..);
                            }
                            
                            // 再移除范围之前的元素
                            if start > 0 {
                                list.drain(0..start as usize);
                            }
                        }

                        Ok(Frame::SimpleString("OK".to_string()))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => Ok(Frame::SimpleString("OK".to_string())),
        }
    }
}