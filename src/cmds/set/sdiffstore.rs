use std::collections::HashSet;

use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Sdiffstore {
    destination: String,
    keys: Vec<String>,
}

impl Sdiffstore {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();

        // At least three parameters are required (command name, target collection key, one or more source collection keys)
        if args.len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'sdiffstore' command"));
        }

        let destination = args[1].to_string();
        let keys = args[2..].iter().map(|arg| arg.to_string()).collect();

        Ok(Sdiffstore { destination, keys })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        if self.keys.is_empty() {
            return Err(Error::msg("ERR wrong number of arguments for 'sdiffstore' command"));
        }

        let mut iter = self.keys.iter();
        let first_key = iter.next().unwrap();

        // get the first collection
        let first_set = match db.records.get(first_key) {
            Some(structure) => {
                match structure {
                    Structure::Set(set) => set.clone(),
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        return Ok(Frame::Error(f.to_string()));
                    }
                }
            },
            None => HashSet::new(), // if the key does not exist, it is considered an empty set
        };

        // calculate the difference set
        let mut difference: HashSet<String> = first_set;
        for key in iter {
            match db.records.get(key) {
                Some(structure) => {
                    match structure {
                        Structure::Set(set) => {
                            // remove elements that exist in the current collection from the difference set
                            for member in set.iter() {
                                difference.remove(member);
                            }
                        },
                        _ => {
                            let f = "ERR Operation against a key holding the wrong kind of value";
                            return Ok(Frame::Error(f.to_string()));
                        }
                    }
                },
                None => {
                    // if the key does not exist, it is considered an empty set and does not affect the difference set calculation
                    continue;
                }
            }
        }

        // store the result to the target key
        db.insert(self.destination.clone(), Structure::Set(difference.clone()));
        
        Ok(Frame::Integer(difference.len() as i64))
    }
}

