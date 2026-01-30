use anyhow::Error;
use crate::frame::Frame;
use crate::server::Handler;

/// Handler 异步命令 trait：专门用于需要 Handler 上下文的异步命令
/// 
/// **这是当前项目中真正使用的异步命令标准接口**，专门用于需要访问 Handler 资源的命令
/// （如 blocking_manager、session_manager）
/// 
/// **设计原则**：
/// - 所有需要 Handler 的异步命令都实现这个 trait
/// - 命令自己负责所有业务逻辑（阻塞、超时、唤醒等）
/// - Handler 只提供资源访问，不包含业务逻辑
/// 
/// **使用示例**：
/// ```rust
/// impl HandlerAsyncCommand for Blpop {
///     fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
///         // 解析逻辑
///     }
///     
///     async fn apply(self, handler: &mut Handler) -> Result<Frame, Error> {
///         // 1. 先尝试非阻塞获取
///         // 2. 如果为空，注册阻塞请求
///         // 3. 使用 tokio::select! 等待结果或超时
///         // 所有逻辑都在这里，不依赖 server.rs
///     }
/// }
/// ```
#[allow(async_fn_in_trait)]
pub trait HandlerAsyncCommand: Sized + Clone {
    /// 从 RESP 帧解析命令
    fn parse_from_frame(frame: Frame) -> Result<Self, Error>;
    
    /// 在 Handler 中异步执行命令
    /// 
    /// 接收 `&mut Handler`，命令自己决定如何使用 Handler 的资源
    /// - 访问 `blocking_manager`：`handler.get_state().blocking_list`
    /// - 访问 `session_manager`：`handler.get_session_manager()`
    /// - 执行数据库命令：`handler.apply_db_command(command).await`
    /// 
    /// 所有阻塞、超时、唤醒等逻辑都应该在这个方法内部完成
    async fn apply(self, handler: &mut Handler) -> Result<Frame, Error>;
}
