use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};
use crate::store::hyperloglog::HyperLogLog;

/**
 * PFCOUNT 命令
 * 
 * 返回 HyperLogLog 的基数估计
 * 
 * 语法: PFCOUNT key [key ...]
 * 
 * 返回值:
 * - 单个 key: 返回该 HyperLogLog 的基数估计
 * - 多个 key: 返回合并后的基数估计（内部合并）
 * - key 不存在: 返回 0
 */
pub struct Pfcount {
    keys: Vec<String>,
}

impl Pfcount {
    
    /**
     * 从 Frame 解析 PFCOUNT 命令
     * 
     * @param frame 命令帧
     * @return Pfcount 实例或错误
     */
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();

        if args.len() < 2 {
            return Err(Error::msg("ERR wrong number of arguments for 'pfcount' command"));
        }

        let keys = args.iter().skip(1).map(|s| s.to_string()).collect();

        Ok(Pfcount { keys })
    }

    /**
     * 执行 PFCOUNT 命令
     * 
     * @param self 命令实例
     * @param db 数据库实例
     * @return 执行结果帧
     */
    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        if self.keys.is_empty() {
            return Ok(Frame::Integer(0));
        }

        // 单个 key 的情况
        if self.keys.len() == 1 {
            match db.records.get_mut(&self.keys[0]) {
                Some(Structure::HyperLogLog(hll)) => {
                    Ok(Frame::Integer(hll.count() as i64))
                },
                Some(_) => {
                    let f = "ERR Operation against a key holding the wrong kind of value";
                    Ok(Frame::Error(f.to_string()))
                },
                None => {
                    // key 不存在，返回 0
                    Ok(Frame::Integer(0))
                }
            }
        } else {
            // 多个 key 的情况：需要合并后计算基数
            let mut merged_hll = HyperLogLog::new();
            let mut has_valid_hll = false;

            for key in &self.keys {
                match db.records.get(key.as_str()) {
                    Some(Structure::HyperLogLog(hll)) => {
                        merged_hll.merge(hll);
                        has_valid_hll = true;
                    },
                    Some(_) => {
                        // 如果某个 key 不是 HyperLogLog 类型，返回错误
                        let f = format!("ERR Operation against a key holding the wrong kind of value: {}", key);
                        return Ok(Frame::Error(f));
                    },
                    None => {
                        // key 不存在，跳过（Redis 的行为）
                    }
                }
            }

            if has_valid_hll {
                Ok(Frame::Integer(merged_hll.count() as i64))
            } else {
                // 所有 key 都不存在，返回 0
                Ok(Frame::Integer(0))
            }
        }
    }
}
