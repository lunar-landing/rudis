use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Bitcount {
    key: String,
    start: Option<isize>,
    end: Option<isize>,
}

impl Bitcount {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let key = frame.get_arg(1);
        
        if key.is_none() {
            return Err(Error::msg("ERR wrong number of arguments for 'bitcount' command"));
        }

        let final_key = key.unwrap().to_string();
        
        let start = frame.get_arg(2).map(|s| {
            s.parse::<isize>().map_err(|_| Error::msg("ERR value is not an integer or out of range"))
        }).transpose()?;

        let end = frame.get_arg(3).map(|s| {
            s.parse::<isize>().map_err(|_| Error::msg("ERR value is not an integer or out of range"))
        }).transpose()?;

        Ok(Bitcount {
            key: final_key,
            start,
            end,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let result_structure = db.get(&self.key);
        
        match result_structure {
            Some(structure) => {
                match structure {
                    Structure::String(value) => {
                        let bytes = value.as_bytes();
                        
                        if bytes.is_empty() {
                            return Ok(Frame::Integer(0));
                        }

                        let (start_byte, end_byte) = match (self.start, self.end) {
                            (None, None) => {
                                // No range is specified, the entire string is calculated
                                (0, bytes.len() as isize - 1)
                            },
                            (Some(s), None) => {
                                // only start end defaults to the end of the string
                                let start = if s < 0 {
                                    (bytes.len() as isize + s).max(0)
                                } else {
                                    s
                                };
                                // if start is out of range returns 0
                                if start >= bytes.len() as isize {
                                    return Ok(Frame::Integer(0));
                                }
                                (start, bytes.len() as isize - 1)
                            },
                            (None, Some(e)) => {
                                // only end start defaults to 0
                                let end = if e < 0 {
                                    (bytes.len() as isize + e).max(0)
                                } else {
                                    e
                                };
                                (0, end)
                            },
                            (Some(s), Some(e)) => {
                                // both are specified
                                let start = if s < 0 {
                                    (bytes.len() as isize + s).max(0)
                                } else {
                                    s
                                };
                                let end = if e < 0 {
                                    (bytes.len() as isize + e).max(0)
                                } else {
                                    e
                                };
                                
                                // if start is out of range returns 0
                                if start >= bytes.len() as isize {
                                    return Ok(Frame::Integer(0));
                                }
                                
                                if start > end {
                                    return Ok(Frame::Integer(0));
                                }
                                
                                (start, end)
                            }
                        };

                        let start_byte = start_byte.max(0) as usize;
                        let end_byte = end_byte.min(bytes.len() as isize - 1).max(0) as usize;

                        if start_byte > end_byte || start_byte >= bytes.len() {
                            return Ok(Frame::Integer(0));
                        }

                        // calculate the number of set bits in the specified range
                        let mut count = 0;
                        for byte in &bytes[start_byte..=end_byte] {
                            count += byte.count_ones();
                        }

                        Ok(Frame::Integer(count as i64))
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

