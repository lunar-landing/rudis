use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};
use crate::store::hyperloglog::HyperLogLog;

/**
 * PFADD 命令
 * 
 * 添加元素到 HyperLogLog 数据结构
 * 
 * 语法: PFADD key [element [element ...]]
 * 
 * 返回值:
 * - 1: 如果至少有一个 HyperLogLog 内部寄存器被更新
 * - 0: 如果没有寄存器被更新
 */
pub struct Pfadd {
    key: String,
    elements: Vec<String>,
}

impl Pfadd {
    
    /**
     * 从 Frame 解析 PFADD 命令
     * 
     * @param frame 命令帧
     * @return Pfadd 实例或错误
     */
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();

        if args.len() < 2 {
            return Err(Error::msg("ERR wrong number of arguments for 'pfadd' command"));
        }

        let key = args[1].to_string();
        let elements = args.iter().skip(2).map(|s| s.to_string()).collect();

        Ok(Pfadd { key, elements })
    }

    /**
     * 执行 PFADD 命令
     * 
     * @param self 命令实例
     * @param db 数据库实例
     * @return 执行结果帧
     */
    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // 检查 key 是否存在
        let key_existed = db.records.contains_key(&self.key);
        
        // 获取或创建 HyperLogLog
        let hll = match db.records.get_mut(&self.key) {
            Some(structure) => {
                match structure {
                    Structure::HyperLogLog(hll) => hll,
                    _ => {
                        let f = "ERR Operation against a key holding the wrong kind of value";
                        return Ok(Frame::Error(f.to_string()));
                    }
                }
            },
            None => {
                // key 不存在，创建新的 HyperLogLog
                let new_hll = HyperLogLog::new();
                db.insert(self.key.clone(), Structure::HyperLogLog(new_hll));
                match db.records.get_mut(&self.key) {
                    Some(Structure::HyperLogLog(hll)) => hll,
                    _ => {
                        return Ok(Frame::Error("ERR Failed to create HyperLogLog".to_string()));
                    }
                }
            }
        };

        // 如果 key 不存在且没有元素，返回 1（因为创建了新的 HyperLogLog）
        if !key_existed && self.elements.is_empty() {
            return Ok(Frame::Integer(1));
        }

        // 添加所有元素
        let mut changed = false;
        for element in self.elements {
            if hll.add(&element) {
                changed = true;
            }
        }

        // 返回 1 如果至少有一个寄存器被更新，否则返回 0
        Ok(Frame::Integer(if changed { 1 } else { 0 }))
    }
}
