use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Msetnx {
    key_vals: Vec<(String, String)>,
}

impl Msetnx {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let args = frame.get_args_from_index(1);

        if args.len() % 2 != 0 {
            return Err(Error::msg("ERR wrong number of arguments for 'msetnx' command"));
        }

        let mut key_vals = Vec::new();
        
        for i in (0..args.len()).step_by(2) {
            let key = args[i].to_string();
            let val = args[i + 1].to_string();
            key_vals.push((key, val));
        }

        Ok(Msetnx { key_vals })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // 首先检查所有键是否都不存在
        for (key, _) in &self.key_vals {
            if db.exists(key) {
                // 如果有任何一个键已经存在，返回 0
                return Ok(Frame::Integer(0));
            }
        }

        // 如果所有键都不存在，则设置所有键值对
        for (key, val) in self.key_vals {
            db.insert(key, Structure::String(val));
        }
        
        // 所有键都成功设置，返回 1
        Ok(Frame::Integer(1))
    }
}