use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};
use crate::store::hyperloglog::HyperLogLog;

/**
 * PFMERGE 命令
 * 
 * 合并多个 HyperLogLog 到一个目标 key
 * 
 * 语法: PFMERGE destkey [sourcekey [sourcekey ...]]
 * 
 * 返回值:
 * - OK: 合并成功
 * 
 * 注意:
 * - 如果目标 key 不存在，会创建一个新的 HyperLogLog
 * - 如果目标 key 存在但不是 HyperLogLog 类型，返回错误
 * - 如果源 key 不存在，会被跳过
 * - 如果源 key 不是 HyperLogLog 类型，返回错误
 */
pub struct Pfmerge {
    destination: String,
    source_keys: Vec<String>,
}

impl Pfmerge {
    
    /**
     * 从 Frame 解析 PFMERGE 命令
     * 
     * @param frame 命令帧
     * @return Pfmerge 实例或错误
     */
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();

        if args.len() < 2 {
            return Err(Error::msg("ERR wrong number of arguments for 'pfmerge' command"));
        }

        let destination = args[1].to_string();
        let source_keys = args.iter().skip(2).map(|s| s.to_string()).collect();

        Ok(Pfmerge { destination, source_keys })
    }

    /**
     * 执行 PFMERGE 命令
     * 
     * @param self 命令实例
     * @param db 数据库实例
     * @return 执行结果帧
     */
    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // 先收集所有源 HyperLogLog 的克隆（避免借用冲突）
        let mut source_hlls = Vec::new();
        
        for key in &self.source_keys {
            match db.records.get(key.as_str()) {
                Some(Structure::HyperLogLog(source_hll)) => {
                    source_hlls.push(source_hll.clone());
                },
                Some(_) => {
                    // 如果源 key 不是 HyperLogLog 类型，返回错误
                    let f = format!("ERR Operation against a key holding the wrong kind of value: {}", key);
                    return Ok(Frame::Error(f));
                },
                None => {
                    // 源 key 不存在，跳过（Redis 的行为）
                }
            }
        }

        // 获取或创建目标 HyperLogLog
        let dest_hll = match db.records.get_mut(&self.destination) {
            Some(Structure::HyperLogLog(hll)) => hll,
            Some(_) => {
                let f = "ERR Operation against a key holding the wrong kind of value";
                return Ok(Frame::Error(f.to_string()));
            },
            None => {
                // 目标 key 不存在，创建新的 HyperLogLog
                let new_hll = HyperLogLog::new();
                db.insert(self.destination.clone(), Structure::HyperLogLog(new_hll));
                match db.records.get_mut(&self.destination) {
                    Some(Structure::HyperLogLog(hll)) => hll,
                    _ => {
                        return Ok(Frame::Error("ERR Failed to create HyperLogLog".to_string()));
                    }
                }
            }
        };

        // 合并所有源 HyperLogLog 到目标
        for source_hll in source_hlls {
            dest_hll.merge(&source_hll);
        }

        Ok(Frame::Ok)
    }
}
