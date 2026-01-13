use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Lrem {
    key: String,
    count: i64,
    value: String,
}

impl Lrem {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let args = frame.get_args();

        if args.len() != 4 {
            return Err(Error::msg("ERR wrong number of arguments for 'lrem' command"));
        }

        let key = args[1].to_string();
        
        let count = args[2].parse::<i64>().map_err(|_| {
            Error::msg("ERR value is not an integer or out of range")
        })?;
        
        let value = args[3].to_string();

        Ok(Lrem { key, count, value })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::List(list) => {
                        let removed_count = self.remove_elements(list);
                        Ok(Frame::Integer(removed_count))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => Ok(Frame::Integer(0)), // 列表不存在时返回 0
        }
    }

    /// 根据 count 的值移除列表中与 value 相等的元素
    /// count > 0: 从表头开始向表尾搜索，移除 count 个
    /// count < 0: 从表尾开始向表头搜索，移除 |count| 个
    /// count = 0: 移除所有与 value 相等的元素
    fn remove_elements(&self, list: &mut Vec<String>) -> i64 {
        let mut removed_count: i64 = 0;

        if self.count > 0 {
            // 从表头开始向表尾搜索
            let mut to_remove = self.count;
            let mut i = 0;
            while i < list.len() && to_remove > 0 {
                if list[i] == self.value {
                    list.remove(i);
                    removed_count += 1;
                    to_remove -= 1;
                } else {
                    i += 1;
                }
            }
        } else if self.count < 0 {
            // 从表尾开始向表头搜索
            let mut to_remove = self.count.abs();
            let mut i = list.len();
            while i > 0 && to_remove > 0 {
                i -= 1;
                if list[i] == self.value {
                    list.remove(i);
                    removed_count += 1;
                    to_remove -= 1;
                }
            }
        } else {
            // count == 0，移除所有与 value 相等的元素
            list.retain(|item| {
                if item == &self.value {
                    removed_count += 1;
                    false
                } else {
                    true
                }
            });
        }

        removed_count
    }
}
