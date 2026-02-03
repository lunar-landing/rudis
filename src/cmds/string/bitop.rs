use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Bitop {
    operation: String,
    dest_key: String,
    keys: Vec<String>,
}

impl Bitop {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let operation = frame.get_arg(1);
        let dest_key = frame.get_arg(2);

        if operation.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'bitop' command"));
        }

        if dest_key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'bitop' command"));
        }

        let operation = operation.unwrap().to_uppercase();
        let dest_key = dest_key.unwrap().to_string();

        // validate the type of action
        if operation != "AND" && operation != "OR" && operation != "XOR" && operation != "NOT" {
            return Err(Error::msg("ERR syntax error"));
        }

        // get all source keys
        let mut keys = Vec::new();
        let args = frame.get_args();
        for i in 3..args.len() {
            keys.push(args[i].clone());
        }

        // The NOT operation accepts only one source key
        if operation == "NOT" && keys.len() != 1 {
            return Err(Error::msg("ERR BITOP NOT must be called with a single source key"));
        }

        // other operations require at least one source key
        if operation != "NOT" && keys.is_empty() {
            return Err(Error::msg("ERR wrong number of arguments for 'bitop' command"));
        }

        Ok(Bitop {
            operation,
            dest_key,
            keys,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // get the values of all source keys
        let mut source_bytes = Vec::new();
        
        for key in &self.keys {
            let result_structure = db.get(key);
            match result_structure {
                Some(structure) => {
                    match structure {
                        Structure::String(value) => {
                            source_bytes.push(value.as_bytes().to_vec());
                        },
                        _ => {
                            return Ok(Frame::Error("WRONGTYPE Operation against a key holding the wrong kind of value".to_string()));
                        }
                    }
                },
                None => {
                    // The key does not exist and is treated as an empty string
                    source_bytes.push(Vec::new());
                }
            }
        }

        if source_bytes.is_empty() {
            return Ok(Frame::Integer(0));
        }

        // find the maximum length
        let max_len = source_bytes.iter().map(|b| b.len()).max().unwrap_or(0);

        if max_len == 0 {
            // all source keys are empty and so are the results
            db.insert(self.dest_key, Structure::String(String::new()));
            return Ok(Frame::Integer(0));
        }

        // perform bit operations
        let result_bytes = match self.operation.as_str() {
            "AND" => {
                Self::bitop_and(&source_bytes, max_len)
            },
            "OR" => {
                Self::bitop_or(&source_bytes, max_len)
            },
            "XOR" => {
                Self::bitop_xor(&source_bytes, max_len)
            },
            "NOT" => {
                Self::bitop_not(&source_bytes[0], max_len)
            },
            _ => {
                return Err(Error::msg("ERR unknown bitop operation"));
            }
        };

        // convert the result to a string
        // note: Redis's string type can store arbitrary byte sequences, not just valid UTF-8
        // so we use unsafe to bypass utf 8 checks
        let result_string = unsafe { String::from_utf8_unchecked(result_bytes) };

        let result_len = result_string.len() as i64;
        db.insert(self.dest_key, Structure::String(result_string));
        Ok(Frame::Integer(result_len))
    }

    fn bitop_and(source_bytes: &[Vec<u8>], max_len: usize) -> Vec<u8> {
        let mut result = vec![0xFF; max_len];
        
        for bytes in source_bytes {
            for (i, &byte) in bytes.iter().enumerate() {
                if i < result.len() {
                    result[i] &= byte;
                }
            }
        }
        
        result
    }

    fn bitop_or(source_bytes: &[Vec<u8>], max_len: usize) -> Vec<u8> {
        let mut result = vec![0; max_len];
        
        for bytes in source_bytes {
            for (i, &byte) in bytes.iter().enumerate() {
                if i < result.len() {
                    result[i] |= byte;
                }
            }
        }
        
        result
    }

    fn bitop_xor(source_bytes: &[Vec<u8>], max_len: usize) -> Vec<u8> {
        let mut result = vec![0; max_len];
        
        for bytes in source_bytes {
            for (i, &byte) in bytes.iter().enumerate() {
                if i < result.len() {
                    result[i] ^= byte;
                }
            }
        }
        
        result
    }

    fn bitop_not(source_bytes: &[u8], max_len: usize) -> Vec<u8> {
        let mut result = vec![0xFF; max_len];
        
        for (i, &byte) in source_bytes.iter().enumerate() {
            if i < result.len() {
                result[i] = !byte;
            }
        }
        
        result
    }
}

