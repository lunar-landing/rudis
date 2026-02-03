use dashmap::DashMap;

use crate::network::{session::Session, session_role::SessionRole};

/// 高性能会话管理器
pub struct SessionManager {
    sessions: DashMap<usize, Session>,
}

impl SessionManager {

    // 创建实例
    pub fn new() -> Self {
        Self {
            sessions: DashMap::new(),
        }
    }

    /// 添加会话
    pub fn create_session(&self, session: Session)  {
        self.sessions.insert(session.get_id(), session);
    }

    /// 移除会话
    pub fn remove_session(&self, session_id: usize) -> bool {
        self.sessions.remove(&session_id).is_some()
    }

    /// 所有会话（Slave）
    pub fn get_slave_sessions(&self) -> Vec<Session> {
        self.sessions.iter()
        .filter(|entry| entry.value().get_role() == &SessionRole::Slave)
        .map(|entry| entry.value().clone())
        .collect()
    }

    /// 获取当前连接数
    pub fn get_connection_count(&self) -> usize {
        self.sessions.len()
    }

    /// 检查是否超过最大连接数限制
    pub fn is_over_max_clients(&self, maxclients: usize) -> bool {
        if maxclients == 0 {
            return false; // 0 表示无限制
        }
        self.get_connection_count() >= maxclients
    }

    /// 获取指定 ID 的会话
    pub fn get_session(&self, session_id: usize) -> Option<Session> {
        self.sessions.get(&session_id).map(|entry| entry.value().clone())
    }
}