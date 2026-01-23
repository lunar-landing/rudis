use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};
use std::collections::HashSet;

pub struct Srandmember {
    key: String,
    count: Option<i64>,
}

impl Srandmember {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let key = frame.get_arg(1);

        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'srandmember' command"));
        }

        let final_key = key.unwrap().to_string();
        
        // count is an optional parameter
        let count = if let Some(count_str) = frame.get_arg(2) {
            match count_str.parse::<i64>() {
                Ok(c) => Some(c),
                Err(_) => return Err(Error::msg("ERR value is not an integer or out of range")),
            }
        } else {
            None
        };

        Ok(Srandmember {
            key: final_key,
            count,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Set(set) => {
                        if set.is_empty() {
                            return Ok(Frame::Null);
                        }

                        let vec: Vec<String> = set.iter().cloned().collect();
                        let vec_len = vec.len();

                        match self.count {
                            None => {
                                // returns a single random element
                                let index = (std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_nanos() as usize) % vec_len;
                                Ok(Frame::BulkString(vec[index].clone()))
                            },
                            Some(count) if count > 0 => {
                                // returns unique random elements up to count
                                let max_count = count.min(vec_len as i64) as usize;
                                let mut result = Vec::new();
                                let mut used_indices = HashSet::new();
                                
                                // generate a random index with timestamps and counters
                                let mut seed = std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_nanos() as usize;
                                
                                while result.len() < max_count && used_indices.len() < vec_len {
                                    seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                                    let index = seed % vec_len;
                                    
                                    if !used_indices.contains(&index) {
                                        used_indices.insert(index);
                                        result.push(Frame::BulkString(vec[index].clone()));
                                    }
                                }
                                Ok(Frame::Array(result))
                            },
                            Some(count) if count < 0 => {
                                // returns random elements that may be duplicated (|count| units)
                                let count = (-count) as usize;
                                let mut result = Vec::new();
                                
                                let mut seed = std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_nanos() as usize;
                                
                                for _ in 0..count {
                                    seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                                    let index = seed % vec_len;
                                    result.push(Frame::BulkString(vec[index].clone()));
                                }
                                Ok(Frame::Array(result))
                            },
                            Some(0) => {
                                Ok(Frame::Array(Vec::new()))
                            },
                            _ => Ok(Frame::Array(Vec::new())),
                        }
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => Ok(Frame::Null),
        }
    }
}

