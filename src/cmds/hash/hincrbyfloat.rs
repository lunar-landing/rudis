use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct HincrbyFloat {
    key: String,
    field: String,
    increment: f64,
}

impl HincrbyFloat {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() != 4 {
            return Err(Error::msg("ERR wrong number of arguments for 'hincrbyfloat' command"));
        }
        
        let key = args[1].to_string();
        let field = args[2].to_string();
        let increment = args[3].parse::<f64>().map_err(|_| {
            Error::msg("ERR value is not a valid float")
        })?;
        
        Ok(HincrbyFloat { key, field, increment })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::Hash(hash) => {
                        // 获取字段的当前值，如果字段不存在则默认为"0"
                        let current_value = hash.get(&self.field).cloned().unwrap_or_else(|| "0".to_string());
                        
                        // 尝试将当前值解析为浮点数
                        match current_value.parse::<f64>() {
                            Ok(num) => {
                                // 计算新值
                                let new_value = num + self.increment;
                                
                                // 更新哈希表中的值
                                hash.insert(self.field, new_value.to_string());
                                
                                // 返回新值
                                Ok(Frame::BulkString(new_value.to_string()))
                            },
                            Err(_) => {
                                // 如果当前值不是浮点数，返回错误
                                let f = "ERR hash value is not a float";
                                Ok(Frame::Error(f.to_string()))
                            }
                        }
                    },
                    _ => {
                        // 键存在但不是哈希类型
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        Ok(Frame::Error(f.to_string()))
                    }
                }
            },
            None => {
                // 键不存在，创建新的哈希表
                let mut hash = std::collections::HashMap::new();
                let new_value = self.increment;
                hash.insert(self.field, new_value.to_string());
                db.insert(self.key.clone(), Structure::Hash(hash));
                
                // 返回新值
                Ok(Frame::BulkString(new_value.to_string()))
            }
        }
    }
}