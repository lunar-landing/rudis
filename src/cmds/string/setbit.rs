use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Setbit {
    key: String,
    offset: usize,
    value: u8,
}

impl Setbit {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let key = frame.get_arg(1);
        let offset = frame.get_arg(2);
        let value = frame.get_arg(3);

        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'setbit' command"));
        }

        if offset.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'setbit' command"));
        }

        if value.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'setbit' command"));
        }

        let final_key = key.unwrap().to_string();
        
        let offset = match offset.unwrap().parse::<usize>() {
            Ok(o) => o,
            Err(_) => return Err(Error::msg("ERR bit offset is not an integer or out of range")),
        };

        let value = match value.unwrap().parse::<u8>() {
            Ok(v) => {
                if v > 1 {
                    return Err(Error::msg("ERR bit is not an integer or out of range"));
                }
                v
            },
            Err(_) => return Err(Error::msg("ERR bit is not an integer or out of range")),
        };

        Ok(Setbit {
            key: final_key,
            offset,
            value,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let result_structure = db.get(&self.key);
        
        let mut bytes = match result_structure {
            Some(structure) => {
                match structure {
                    Structure::String(value) => {
                        value.as_bytes().to_vec()
                    },
                    _ => {
                        return Ok(Frame::Error("WRONGTYPE Operation against a key holding the wrong kind of value".to_string()));
                    }
                }
            },
            None => {
                Vec::new()
            }
        };

        // Calculate the number of bytes and bit offsets required
        let byte_index = self.offset / 8;
        let bit_offset = 7 - (self.offset % 8); // Redis uses a large end order, with the highest position first

        // extend the byte array if needed
        while bytes.len() <= byte_index {
            bytes.push(0);
        }

        // get the old value
        let old_bit = if byte_index < bytes.len() {
            (bytes[byte_index] >> bit_offset) & 1
        } else {
            0
        };

        // set the new value
        if self.value == 1 {
            bytes[byte_index] |= 1 << bit_offset;
        } else {
            bytes[byte_index] &= !(1 << bit_offset);
        }

        // convert byte arrays back to strings
        // note: Redis's string type can store arbitrary byte sequences, not just valid UTF-8
        // so we use unsafe to bypass utf 8 checks
        let new_value = unsafe { String::from_utf8_unchecked(bytes) };

        db.insert(self.key, Structure::String(new_value));
        Ok(Frame::Integer(old_bit as i64))
    }
}

