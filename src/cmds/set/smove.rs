use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Smove {
    source: String,
    destination: String,
    member: String,
}

impl Smove {
    
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();

        // four parameters are required: command name, source collection key, target collection key, and member
        if args.len() != 4 {
            return Err(Error::msg("ERR wrong number of arguments for 'smove' command"));
        }

        let source = args[1].to_string();
        let destination = args[2].to_string();
        let member = args[3].to_string();

        Ok(Smove { source, destination, member })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // check if the source collection exists and contains the member
        let member_exists = match db.records.get_mut(&self.source) {
            Some(structure) => {
                match structure {
                    Structure::Set(set) => {
                        // check for members to exist and remove them from the source collection
                        set.remove(&self.member)
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        return Ok(Frame::Error(f.to_string()));
                    }
                }
            },
            None => false, // the source collection does not exist, and neither do the members
        };

        // if the member does not exist in the source collection, returns 0
        if !member_exists {
            return Ok(Frame::Integer(0));
        }

        // add members to the target collection
        match db.records.get_mut(&self.destination) {
            Some(structure) => {
                match structure {
                    Structure::Set(set) => {
                        // if the member already exists in the target collection, insert returns false, but we still return 1
                        set.insert(self.member.clone());
                        Ok(Frame::Integer(1))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                // the target collection does not exist, create a new collection
                use std::collections::HashSet;
                let mut set = HashSet::new();
                set.insert(self.member);
                db.insert(self.destination, Structure::Set(set));
                Ok(Frame::Integer(1))
            }
        }
    }
}

