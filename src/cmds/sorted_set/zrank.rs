use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Zrank {
    key: String,
    member: String,
}

impl Zrank {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() != 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'zrank' command"));
        }
        let key = args[1].to_string(); // 键
        let member = args[2].to_string(); // 成员
        Ok(Zrank { key, member })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.records.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::SortedSet(set) => {
                        // 使用跳表计算排名，O(log n) 时间复杂度
                        if let Some(rank) = set.rank(&self.member) {
                            Ok(Frame::Integer(rank as i64))
                        } else {
                            // 如果成员不存在，返回 nil
                            Ok(Frame::Null)
                        }
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                // 如果键不存在，返回 nil
                Ok(Frame::Null)
            }
        }
    }
}