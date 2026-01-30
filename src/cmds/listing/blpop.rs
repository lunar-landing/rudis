use anyhow::Error;
use crate::{frame::Frame, server::Handler};
use crate::cmds::async_command::HandlerAsyncCommand;

/// BLPOP 命令：阻塞式从列表左端弹出元素
/// 
/// 这个命令需要在 Handler 中处理，因为：
/// 1. 需要异步等待（tokio::select!）
/// 2. 需要访问 blocking_manager 和 session_manager
/// 3. 需要跨任务通信
/// 
/// **实现标准**：实现了 `HandlerAsyncCommand` trait，所有逻辑都在 `apply` 方法中
#[derive(Clone)]
pub struct Blpop {
    keys: Vec<String>,
    timeout: u64,  // 秒，0 表示永久阻塞（实际限制为 3600 秒）
}

impl HandlerAsyncCommand for Blpop {
    fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        
        if args.len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'blpop' command"));
        }

        // 安全限制：防止恶意客户端发送过多的 key
        if args.len() - 2 > 1000 {
            return Err(Error::msg("ERR too many keys for 'blpop' command"));
        }
        
        let keys: Vec<String> = args[1..args.len()-1]
            .iter()
            .map(|s| s.to_string())
            .collect();
        
        let timeout = args.last()
            .ok_or_else(|| Error::msg("ERR timeout required"))?
            .parse::<u64>()
            .map_err(|_| Error::msg("ERR timeout is not an integer"))?;
        
        Ok(Blpop { keys, timeout })
    }

    /// 在 Handler 中执行 BLPOP 命令
    /// 
    /// 流程：
    /// 1. 先非阻塞检查数据库，如果列表非空，立即执行 LPOP
    /// 2. 如果列表为空，注册阻塞请求
    /// 3. 使用 tokio::select! 等待结果或超时
    /// 
    /// 所有逻辑都在这里，不依赖 server.rs
    async fn apply(self, handler: &mut Handler) -> Result<Frame, Error> {
        use crate::store::blocking::BlockDirection;
        use tokio::time::{sleep, Duration};
        
        // 1. 先尝试非阻塞获取：检查第一个键是否非空
        let first_key = &self.keys[0];
        let lpop_frame = Frame::Array(vec![
            Frame::BulkString("LPOP".to_string()),
            Frame::BulkString(first_key.clone()),
        ]);
        
        let lpop_cmd = match crate::command::Command::parse_from_frame(lpop_frame) {
            Ok(cmd) => cmd,
            Err(_) => return Ok(Frame::Error("Internal error".to_string())),
        };
        
        // 非阻塞执行 LPOP
        let immediate_result = handler.apply_db_command(lpop_cmd).await?;
        
        // 如果列表非空，直接返回结果
        if !matches!(immediate_result, Frame::Null) {
            // 将 LPOP 的结果转换为 BLPOP 的格式 [key, value]
            if let Frame::BulkString(value) = immediate_result {
                return Ok(Frame::Array(vec![
                    Frame::BulkString(first_key.clone()),
                    Frame::BulkString(value),
                ]));
            }
        }
        
        // 2. 列表为空，注册阻塞请求
        let timeout = if self.timeout == 0 {
            Some(Duration::from_secs(3600)) // 设置合理上限
        } else {
            Some(Duration::from_secs(self.timeout))
        };
        
        let mut blocking_manager = handler.get_state().blocking_list.lock().await;
        let session_id = handler.get_session().get_id();
        let receiver = blocking_manager.register_blocking_request(
            self.keys.clone(),
            session_id,
            BlockDirection::Left,
            timeout,
        );
        drop(blocking_manager); // 释放锁，避免在等待时持有锁
        
        // 3. 使用 tokio::select! 处理超时
        let timeout_duration = timeout.unwrap_or(Duration::from_secs(3600));
        let blocking_manager_clone = handler.get_state().blocking_list.clone();
        
        tokio::select! {
            result = receiver => {
                match result {
                    Ok(frame) => Ok(frame),
                    Err(_) => Ok(Frame::Null),
                }
            }
            _ = sleep(timeout_duration) => {
                // 超时，清理请求
                let mut manager = blocking_manager_clone.lock().await;
                manager.cleanup_session(session_id);
                Ok(Frame::Null)
            }
        }
    }
}

