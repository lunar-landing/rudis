use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Getbit {
    key: String,
    offset: usize,
}

impl Getbit {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let key = frame.get_arg(1);
        let offset = frame.get_arg(2);

        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'getbit' command"));
        }

        if offset.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'getbit' command"));
        }

        let final_key = key.unwrap().to_string();
        
        let offset = match offset.unwrap().parse::<usize>() {
            Ok(o) => o,
            Err(_) => return Err(Error::msg("ERR bit offset is not an integer or out of range")),
        };

        Ok(Getbit {
            key: final_key,
            offset,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let result_structure = db.get(&self.key);
        
        match result_structure {
            Some(structure) => {
                match structure {
                    Structure::String(value) => {
                        let bytes = value.as_bytes();
                        
                        let byte_index = self.offset / 8;
                        let bit_offset = 7 - (self.offset % 8); // Redis uses a large end order, with the highest position first

                        if byte_index >= bytes.len() {
                            Ok(Frame::Integer(0))
                        } else {
                            let bit = (bytes[byte_index] >> bit_offset) & 1;
                            Ok(Frame::Integer(bit as i64))
                        }
                    },
                    _ => {
                        Ok(Frame::Error("WRONGTYPE Operation against a key holding the wrong kind of value".to_string()))
                    }
                }
            },
            None => {
                Ok(Frame::Integer(0))
            }
        }
    }
}

