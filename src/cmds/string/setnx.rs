use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Setnx {
    key: String,
    value: String,
}

impl Setnx {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let key = frame.get_arg(1);
        let value = frame.get_arg(2);

        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'setnx' command"));
        }

        if value.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'setnx' command"));
        }

        let final_key = key.unwrap().to_string();
        let final_value = value.unwrap().to_string();

        Ok(Setnx {
            key: final_key,
            value: final_value,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // check if the key is present
        if db.exists(&self.key) {
            Ok(Frame::Integer(0))
        } else {
            db.insert(self.key, Structure::String(self.value));
            Ok(Frame::Integer(1))
        }
    }
}

