use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Setex {
    key: String,
    seconds: u64,
    value: String,
}

impl Setex {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let key = frame.get_arg(1);
        let seconds = frame.get_arg(2);
        let value = frame.get_arg(3);

        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'setex' command"));
        }

        if seconds.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'setex' command"));
        }

        if value.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'setex' command"));
        }

        let final_key = key.unwrap().to_string();
        let final_value = value.unwrap().to_string();
        
        let seconds = match seconds.unwrap().parse::<u64>() {
            Ok(s) => s,
            Err(_) => return Err(Error::msg("ERR value is not an integer or out of range")),
        };

        Ok(Setex {
            key: final_key,
            seconds,
            value: final_value,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // if the key already exists, clear the old expired records first
        if db.records.contains_key(&self.key) {
            db.expire_records.remove(&self.key);
        }
        db.insert(self.key.clone(), Structure::String(self.value));
        db.expire(self.key, self.seconds * 1000); // convert to milliseconds
        Ok(Frame::Ok)
    }
}

