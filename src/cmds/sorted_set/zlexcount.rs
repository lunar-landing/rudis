use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Zlexcount {
    key: String,
    min: String,
    max: String,
}

impl Zlexcount {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() != 4 {
            return Err(Error::msg("ERR wrong number of arguments for 'zlexcount' command"));
        }
        let key = args[1].to_string(); // 键
        let min = args[2].to_string(); // 最小区间
        let max = args[3].to_string(); // 最大区间
        Ok(Zlexcount { key, min, max })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.records.get(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::SortedSet(set) => {
                        // 解析区间
                        let min_bound = self.parse_bound(&self.min, true)?;
                        let max_bound = self.parse_bound(&self.max, false)?;
                        
                        // 检查 min > max 的情况
                        if self.is_range_invalid(&min_bound, &max_bound) {
                            return Ok(Frame::Integer(0));
                        }
                        
                        // 计数符合条件的成员（按字典序）
                        let count = set.members_lex()
                            .iter()
                            .filter(|member| {
                                self.is_in_range(member, &min_bound, &max_bound)
                            })
                            .count();
                        
                        Ok(Frame::Integer(count as i64))
                    },
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                Ok(Frame::Integer(0)) // 如果键不存在，返回 0
            }
        }
    }
    
    // 解析区间边界
    fn parse_bound(&self, bound: &str, _is_min: bool) -> Result<Bound, Error> {
        if bound == "-" {
            return Ok(Bound::NegInf);
        }
        if bound == "+" {
            return Ok(Bound::PosInf);
        }
        
        if bound.starts_with('[') {
            // 闭区间
            let value = &bound[1..];
            Ok(Bound::Inclusive(value.to_string()))
        } else if bound.starts_with('(') {
            // 开区间
            let value = &bound[1..];
            Ok(Bound::Exclusive(value.to_string()))
        } else {
            Err(Error::msg(format!("ERR min or max not valid string range item")))
        }
    }
    
    // 检查范围是否无效（min > max）
    fn is_range_invalid(&self, min: &Bound, max: &Bound) -> bool {
        match (min, max) {
            // 如果 min 是正无穷或 max 是负无穷，范围无效
            (Bound::PosInf, _) | (_, Bound::NegInf) => true,
            // 比较两个具体的值
            (Bound::Inclusive(min_val), Bound::Inclusive(max_val)) => min_val > max_val,
            (Bound::Inclusive(min_val), Bound::Exclusive(max_val)) => min_val >= max_val,
            (Bound::Exclusive(min_val), Bound::Inclusive(max_val)) => min_val >= max_val,
            (Bound::Exclusive(min_val), Bound::Exclusive(max_val)) => min_val >= max_val,
            // 其他情况（包含 -inf、+inf）范围有效
            _ => false,
        }
    }
    fn is_in_range(&self, member: &str, min: &Bound, max: &Bound) -> bool {
        // 检查最小边界
        let meets_min = match min {
            Bound::NegInf => true,
            Bound::PosInf => false,
            Bound::Inclusive(val) => member >= val.as_str(),
            Bound::Exclusive(val) => member > val.as_str(),
        };
        
        // 检查最大边界
        let meets_max = match max {
            Bound::NegInf => false,
            Bound::PosInf => true,
            Bound::Inclusive(val) => member <= val.as_str(),
            Bound::Exclusive(val) => member < val.as_str(),
        };
        
        meets_min && meets_max
    }
}

// 区间边界类型
enum Bound {
    NegInf,              // 负无穷 (-)
    PosInf,              // 正无穷 (+)
    Inclusive(String),   // 闭区间 [value
    Exclusive(String),   // 开区间 (value
}
