use std::collections::HashSet;

use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Sinterstore {
    destination: String,
    keys: Vec<String>,
}

impl Sinterstore {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();

        // at least three parameters are required (command name, target collection key, one or more source collection keys)
        if args.len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'sinterstore' command"));
        }

        let destination = args[1].to_string();
        let keys = args[2..].iter().map(|arg| arg.to_string()).collect();

        Ok(Sinterstore { destination, keys })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let mut iter = self.keys.iter();
        let first = iter.next().unwrap();

        let intersection = match db.records.get(first) {
            Some(structure) => {
                match structure {
                    Structure::Set(first_set) => {       
                        let mut intersection: HashSet<String> = first_set.clone();
                        for key in iter {
                            match db.records.get(key) {
                                Some(structure) => {
                                    match structure {
                                        Structure::Set(set) => {
                                            intersection = intersection.intersection(set).cloned().collect();
                                        },
                                        _ => {
                                            let f = "ERR Operation against a key holding the wrong kind of value";
                                            return Ok(Frame::Error(f.to_string()));
                                        }
                                    }
                                },
                                None => {
                                    // if any of the keys do not exist, the intersection is empty
                                    intersection.clear();
                                    break;
                                }
                            }
                        }
                        intersection
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        return Ok(Frame::Error(f.to_string()));
                    }
                }
            },
            None => {
                // if the first key does not exist, the intersection is empty
                HashSet::new()
            }
        };

        // store the result to the target key
        db.insert(self.destination.clone(), Structure::Set(intersection.clone()));
        
        Ok(Frame::Integer(intersection.len() as i64))
    }
}

