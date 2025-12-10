use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame, tools::pattern};

pub struct Sscan {
    key: String,
    cursor: u64,
    pattern: Option<String>,
    count: Option<u64>,
}

impl Sscan {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args_from_index(1);
        if args.len() < 2 {
            return Err(Error::msg("SSCAN command requires at least two arguments"));
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

        Ok(Sscan { key, cursor, pattern, count })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // 默认匹配模式为 "*"
        let pattern = self.pattern.unwrap_or_else(|| "*".to_string());
        // 默认返回数量为 10
        let count = self.count.unwrap_or(10) as usize;

        match db.records.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Set(set) => {
                        // 获取所有匹配的成员
                        let matched_members: Vec<String> = set.iter()
                            .filter(|member| pattern::is_match(member, &pattern))
                            .cloned()
                            .collect();

                        // 根据游标确定返回的成员
                        let start_index = self.cursor as usize;
                        let end_index = std::cmp::min(start_index + count, matched_members.len());

                        // 获取要返回的成员
                        let members_to_return = if start_index < matched_members.len() {
                            matched_members[start_index..end_index].to_vec()
                        } else {
                            vec![]
                        };

                        // 计算下一个游标
                        let next_cursor = if end_index >= matched_members.len() {
                            0  // 如果已经遍历完所有成员，返回0表示结束
                        } else {
                            end_index as u64  // 否则返回下一个位置作为游标
                        };

                        // 构造返回结果：第一个元素是游标，第二个元素是成员数组
                        let members_frames: Vec<Frame> = members_to_return.into_iter().map(Frame::BulkString).collect();
                        let result_array = vec![
                            Frame::Integer(next_cursor as i64),
                            Frame::Array(members_frames),
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
                // 如果键不存在，返回空数组和游标0
                let result_array = vec![
                    Frame::Integer(0),
                    Frame::Array(vec![]),
                ];
                Ok(Frame::Array(result_array))
            }
        }
    }
}