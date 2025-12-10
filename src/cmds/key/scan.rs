use anyhow::Error;
use crate::{store::db::Db, frame::Frame, tools::pattern};

pub struct Scan {
    cursor: u64,
    pattern: Option<String>,
    count: Option<u64>,
}

impl Scan {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args_from_index(1);
        if args.is_empty() {
            return Err(Error::msg("SCAN command requires at least one argument"));
        }

        let cursor = args[0].parse::<u64>()?;

        let mut pattern = None;
        let mut count = None;

        let mut i = 1;
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

        Ok(Scan { cursor, pattern, count })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // 默认匹配模式为 "*"
        let pattern = self.pattern.unwrap_or_else(|| "*".to_string());
        // 默认返回数量为 10
        let count = self.count.unwrap_or(10) as usize;

        // 获取所有匹配的键
        let matched_keys: Vec<String> = db.records.keys()
            .filter(|key| pattern::is_match(key, &pattern))
            .cloned()
            .collect();

        // 根据游标确定返回的键
        let start_index = self.cursor as usize;
        let end_index = std::cmp::min(start_index + count, matched_keys.len());

        // 获取要返回的键
        let keys_to_return = if start_index < matched_keys.len() {
            matched_keys[start_index..end_index].to_vec()
        } else {
            vec![]
        };

        // 计算下一个游标
        let next_cursor = if end_index >= matched_keys.len() {
            0  // 如果已经遍历完所有键，返回0表示结束
        } else {
            end_index as u64  // 否则返回下一个位置作为游标
        };

        // 构造返回结果：第一个元素是游标，第二个元素是键数组
        let keys_frames: Vec<Frame> = keys_to_return.into_iter().map(Frame::BulkString).collect();
        let result_array = vec![
            Frame::Integer(next_cursor as i64),
            Frame::Array(keys_frames),
        ];

        Ok(Frame::Array(result_array))
    }
}