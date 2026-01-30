use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame, tools::pattern};

pub struct Hscan {
    key: String,
    cursor: u64,
    pattern: Option<String>,
    count: Option<u64>,
}

impl Hscan {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args_from_index(1);
        if args.len() < 2 {
            return Err(Error::msg("HSCAN command requires at least two arguments"));
        }

        let key = args[0].clone();
        let cursor = args[1].parse::<u64>()?;

        let mut pattern = None;
        let mut count = None;

        let mut i = 2;
        while i < args.len() {
            let arg = &args[i].to_uppercase();
            if arg == "MATCH" {
                if i + 1 >= args.len() {
                    return Err(Error::msg("MATCH option requires an argument"));
                }
                pattern = Some(args[i + 1].clone());
                i += 2;
            } else if arg == "COUNT" {
                if i + 1 >= args.len() {
                    return Err(Error::msg("COUNT option requires an argument"));
                }
                count = Some(args[i + 1].parse::<u64>()?);
                i += 2;
            } else {
                return Err(Error::msg(format!("Unknown option: {}", args[i])));
            }
        }

        Ok(Hscan { key, cursor, pattern, count })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // the default matching mode is
        let pattern = self.pattern.unwrap_or_else(|| "*".to_string());
        // the default return quantity is 10
        let count = self.count.unwrap_or(10) as usize;

        match db.records.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Hash(hash) => {
                        // get all matching field value pairs
                        let matched_pairs: Vec<(String, String)> = hash.iter()
                            .filter(|(field, _)| pattern::is_match(field, &pattern))
                            .map(|(field, value)| (field.clone(), value.clone()))
                            .collect();

                        // determine the field-value pair returned based on the cursor
                        let start_index = self.cursor as usize;
                        let end_index = std::cmp::min(start_index + count, matched_pairs.len());

                        // get the field value pair you want to return
                        let pairs_to_return = if start_index < matched_pairs.len() {
                            matched_pairs[start_index..end_index].to_vec()
                        } else {
                            vec![]
                        };

                        // calculate the next cursor
                        let next_cursor = if end_index >= matched_pairs.len() {
                            0  // if all fields have been traversed, returning 0 means it's over
                        } else {
                            end_index as u64  // otherwise return to the next position as the cursor
                        };

                        // the construct returns the result: the first element is the cursor and the second element is the field-value pair array
                        // HSCAN return to formatting：[cursor, [field1, value1, field2, value2, ...]]
                        let mut pairs_frames = Vec::new();
                        for (field, value) in pairs_to_return {
                            pairs_frames.push(Frame::BulkString(field));
                            pairs_frames.push(Frame::BulkString(value));
                        }
                        
                        let result_array = vec![
                            Frame::Integer(next_cursor as i64),
                            Frame::Array(pairs_frames),
                        ];

                        Ok(Frame::Array(result_array))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                // if the key does not exist, return an empty array and cursor 0
                let result_array = vec![
                    Frame::Integer(0),
                    Frame::Array(vec![]),
                ];
                Ok(Frame::Array(result_array))
            }
        }
    }
}

