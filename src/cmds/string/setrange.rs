use anyhow::Error;

use crate::{
    frame::Frame,
    store::db::{Db, Structure},
};

pub struct SetRange {
    key: String,
    offset: i64,
    value: String,
}

impl SetRange {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let key = frame.get_arg(1);
        let offset = frame.get_arg(2);
        let value = frame.get_arg(3);

        if key.is_none() || offset.is_none() || value.is_none() {
            return Err(Error::msg(
                "ERR wrong number of arguments for 'setrange' command",
            ));
        }

        let final_key = key.unwrap().to_string();
        let final_offset = offset.unwrap().to_string();
        let final_value = value.unwrap().to_string();

        let offset_int = match final_offset.parse::<i64>() {
            Ok(n) => n,
            Err(_) => return Err(Error::msg("ERR value is not an integer or out of range")),
        };

        if offset_int < 0 {
            return Err(Error::msg(
                "ERR offset is out of range, must be positive",
            ));
        }

        Ok(SetRange {
            key: final_key,
            offset: offset_int,
            value: final_value,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // 获取当前值，如果不存在则创建一个空字符串
        let current_value = match db.get(&self.key) {
            Some(Structure::String(s)) => s.clone(),
            Some(_) => {
                return Err(Error::msg(
                    "WRONGTYPE Operation against a key holding the wrong kind of value"
                ))
            }
            None => String::new(),
        };

        // 将字符串转换为字节向量以便操作
        let mut bytes = current_value.into_bytes();
        let offset = self.offset as usize;
        let value_bytes = self.value.into_bytes();

        // 确保字节数组足够长以容纳新数据
        if bytes.len() < offset + value_bytes.len() {
            bytes.resize(offset + value_bytes.len(), 0);
        }

        // 在指定偏移处写入新值
        for (i, byte) in value_bytes.iter().enumerate() {
            bytes[offset + i] = *byte;
        }

        // 转换回字符串并保存
        let new_value = match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => {
                return Err(Error::msg(
                    "ERR invalid UTF-8 sequence produced by SETRANGE operation"
                ))
            }
        };

        // 保存到数据库
        db.insert(self.key.clone(), Structure::String(new_value));
        
        // 返回修改后的字符串长度
        let length = db.get(&self.key).map_or(0, |s| {
            if let Structure::String(str) = s {
                str.len()
            } else {
                0
            }
        });

        Ok(Frame::Integer(length as i64))
    }
}