use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Psetex {
    key: String,
    milliseconds: u64,
    value: String,
}

impl Psetex {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let key = frame.get_arg(1);
        let milliseconds = frame.get_arg(2);
        let value = frame.get_arg(3);

        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'psetex' command"));
        }

        if milliseconds.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'psetex' command"));
        }

        if value.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'psetex' command"));
        }

        let final_key = key.unwrap().to_string();
        let final_value = value.unwrap().to_string();
        
        let milliseconds = match milliseconds.unwrap().parse::<u64>() {
            Ok(ms) => ms,
            Err(_) => return Err(Error::msg("ERR value is not an integer or out of range")),
        };

        Ok(Psetex {
            key: final_key,
            milliseconds,
            value: final_value,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // if the key already exists, clear the old expired records first
        if db.records.contains_key(&self.key) {
            db.expire_records.remove(&self.key);
        }
        db.insert(self.key.clone(), Structure::String(self.value));
        db.expire(self.key, self.milliseconds); // 已经是毫秒
        Ok(Frame::Ok)
    }
}

