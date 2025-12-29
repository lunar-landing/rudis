use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::oneshot;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
};

use crate::args::Args;
use crate::command::Command;
use crate::frame::Frame;
use crate::store::db::DatabaseMessage;
use crate::store::db_manager::DatabaseManager;

/// Web服务器
pub struct WebServer {
    args: Arc<Args>,
    db_manager: Arc<DatabaseManager>,
}

impl WebServer {
    
    pub fn new(args: Arc<Args>, db_manager: Arc<DatabaseManager>) -> Self {
        WebServer {
            args,
            db_manager,
        }
    }

    pub async fn start(self, port: u16) {
        let max_databases = self.args.databases;
        let bind_addr = format!("{}:{}", self.args.bind, port);
        let web_state = Arc::new(WebState {
            db_manager: self.db_manager,
            max_databases,
            webuser: self.args.webuser.clone(),
            webpass: self.args.webpass.clone(),
        });
        
        let web_router = create_router(web_state);
        axum::Server::bind(&bind_addr.parse().unwrap()).serve(web_router.into_make_service()).await.expect("Web server failed to start");
    }
}

/// Web服务状态
pub struct WebState {
    pub db_manager: Arc<DatabaseManager>,
    pub max_databases: usize,
    pub webuser: String,
    pub webpass: String,
}

/// 数据库信息
#[derive(Serialize)]
struct DatabaseInfo {
    id: usize,
    name: String,
    key_count: usize,
}

/// 键信息
#[derive(Serialize)]
struct KeyInfo {
    key: String,
    #[serde(rename = "type")]
    key_type: String,
    ttl: i64,
}

/// 键值响应
#[derive(Serialize)]
struct KeyValueResponse {
    key: String,
    #[serde(rename = "type")]
    key_type: String,
    value: serde_json::Value,
    ttl: i64,
}

/// 设置键值请求
#[derive(Deserialize)]
struct SetKeyRequest {
    value: String,
    ttl: Option<u64>,
}

/// 更新TTL请求
#[derive(Deserialize)]
struct UpdateTtlRequest {
    ttl: u64,
}

/// 登录请求
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

/// CLI命令请求
#[derive(Deserialize)]
struct CliRequest {
    command: String,
    db: Option<usize>,
}

/// 查询参数
#[derive(Deserialize)]
struct ListKeysQuery {
    pattern: Option<String>,
    db: Option<usize>,
}

/// 创建Web路由
fn create_router(state: Arc<WebState>) -> Router {
    Router::new()
        .route("/api/login", post(login))
        .route("/api/stats", get(get_stats))
        .route("/api/databases", get(list_databases))
        .route("/api/keys", get(list_keys))
        .route("/api/keys/:key", get(get_key_value))
        .route("/api/keys/:key", delete(delete_key))
        .route("/api/keys/:key/ttl", put(update_key_ttl))
        .route("/api/keys/:key", post(set_key_value))
        .route("/api/cli", post(execute_cli))
        
        // 静态文件服务
        .nest_service("/", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// 登录验证
async fn login(
    State(state): State<Arc<WebState>>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    if req.username == state.webuser && req.password == state.webpass {
        Json(json!({
            "success": true,
            "message": "登录成功"
        }))
    } else {
        Json(json!({
            "success": false,
            "message": "用户名或密码错误"
        }))
    }
}

/// 执行CLI命令
async fn execute_cli(
    State(state): State<Arc<WebState>>,
    Json(req): Json<CliRequest>,
) -> impl IntoResponse {
    let db_id = req.db.unwrap_or(0);
    if db_id >= state.max_databases {
        return Json(json!({
            "success": false,
            "result": "数据库索引超出范围"
        }));
    }

    // 解析命令
    let parts: Vec<&str> = req.command.trim().split_whitespace().collect();
    if parts.is_empty() {
        return Json(json!({
            "success": false,
            "result": "命令不能为空"
        }));
    }

    let frame = Frame::Array(
        parts.iter().map(|s| Frame::BulkString(s.to_string())).collect()
    );

    let command = match Command::parse_from_frame(frame) {
        Ok(cmd) => cmd,
        Err(e) => {
            return Json(json!({
                "success": false,
                "result": format!("(error) {}", e)
            }));
        }
    };

    let sender = state.db_manager.get_sender(db_id);
    let (tx, rx) = oneshot::channel();
    let message = DatabaseMessage::Command { sender: tx, command };

    if sender.send(message).await.is_err() {
        return Json(json!({
            "success": false,
            "result": "(error) 发送命令失败"
        }));
    }

    match rx.await {
        Ok(result_frame) => {
            let result = format_frame_result(&result_frame);
            Json(json!({
                "success": true,
                "result": result
            }))
        }
        Err(_) => Json(json!({
            "success": false,
            "result": "(error) 接收响应失败"
        }))
    }
}

/// 格式化Frame结果
fn format_frame_result(frame: &Frame) -> String {
    match frame {
        Frame::SimpleString(s) => s.clone(),
        Frame::BulkString(s) => format!("\"{}\"", s),
        Frame::Integer(i) => format!("(integer) {}", i),
        Frame::Null => "(nil)".to_string(),
        Frame::Error(e) => format!("(error) {}", e),
        Frame::Ok => "OK".to_string(),
        Frame::RDBFile(_) => "(rdb file)".to_string(),
        Frame::Array(arr) => {
            if arr.is_empty() {
                "(empty array)".to_string()
            } else {
                arr.iter()
                    .enumerate()
                    .map(|(i, f)| format!("{}) {}", i + 1, format_frame_result(f)))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        }
    }
}

/// 获取服务器统计信息
async fn get_stats(State(state): State<Arc<WebState>>) -> impl IntoResponse {
    let mut total_keys = 0;
    let mut total_memory = 0;
    
    // 统计所有数据库的键数量和内存
    for db_id in 0..state.max_databases {
        let sender = state.db_manager.get_sender(db_id);
        
        // 获取DBSIZE
        let frame = Frame::Array(vec![Frame::BulkString("DBSIZE".to_string())]);
        if let Ok(command) = Command::parse_from_frame(frame) {
            let (tx, rx) = oneshot::channel();
            let message = DatabaseMessage::Command { sender: tx, command };
            if sender.send(message).await.is_ok() {
                if let Ok(Frame::Integer(count)) = rx.await {
                    total_keys += count as usize;
                    total_memory += count as usize * 100; // 估算内存
                }
            }
        }
    }
    
    Json(json!({
        "success": true,
        "data": {
            "connected_clients": 1,
            "total_keys": total_keys,
            "used_memory": total_memory,
            "used_memory_human": format_memory(total_memory),
            "databases": state.max_databases
        }
    }))
}

/// 格式化内存大小
fn format_memory(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2}KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.2}MB", bytes as f64 / 1024.0 / 1024.0)
    } else {
        format!("{:.2}GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0)
    }
}

/// 列出所有数据库
async fn list_databases(State(state): State<Arc<WebState>>) -> impl IntoResponse {
    let mut databases = Vec::new();
    
    for db_id in 0..state.max_databases {
        let sender = state.db_manager.get_sender(db_id);
        
        // 获取DBSIZE
        let frame = Frame::Array(vec![Frame::BulkString("DBSIZE".to_string())]);
        let command = match Command::parse_from_frame(frame) {
            Ok(cmd) => cmd,
            Err(_) => continue,
        };
        
        let (tx, rx) = oneshot::channel();
        let message = DatabaseMessage::Command { sender: tx, command };
        
        if sender.send(message).await.is_ok() {
            if let Ok(result_frame) = rx.await {
                let key_count = match result_frame {
                    Frame::Integer(count) => count as usize,
                    _ => 0,
                };
                
                databases.push(DatabaseInfo {
                    id: db_id,
                    name: format!("db{}", db_id),
                    key_count,
                });
            }
        }
    }
    
    Json(json!({
        "success": true,
        "data": databases
    }))
}

/// 列出指定数据库的所有键
async fn list_keys(
    State(state): State<Arc<WebState>>,
    Query(params): Query<ListKeysQuery>,
) -> impl IntoResponse {
    let db_id = params.db.unwrap_or(0);
    let pattern = params.pattern.unwrap_or_else(|| "*".to_string());
    
    if db_id >= state.max_databases {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "数据库索引超出范围"
            }))
        );
    }
    
    let sender = state.db_manager.get_sender(db_id);
    
    // 执行KEYS命令
    let frame = Frame::Array(vec![
        Frame::BulkString("KEYS".to_string()),
        Frame::BulkString(pattern),
    ]);
    
    let command = match Command::parse_from_frame(frame) {
        Ok(cmd) => cmd,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("解析命令失败: {}", e)
                }))
            );
        }
    };
    
    let (tx, rx) = oneshot::channel();
    let message = DatabaseMessage::Command { sender: tx, command };
    
    if sender.send(message).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": "发送命令失败"
            }))
        );
    }
    
    match rx.await {
        Ok(result_frame) => {
            let keys = match result_frame {
                Frame::Array(frames) => {
                    frames.into_iter().filter_map(|f| {
                        if let Frame::BulkString(key) = f {
                            Some(key)
                        } else {
                            None
                        }
                    }).collect::<Vec<_>>()
                },
                _ => vec![],
            };
            
            // 获取每个键的详细信息
            let mut key_infos = Vec::new();
            for key in keys {
                if let Some(info) = get_key_info(&state, db_id, &key).await {
                    key_infos.push(info);
                }
            }
            
            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "data": key_infos
                }))
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": format!("接收响应失败: {}", e)
            }))
        ),
    }
}

/// 获取键的详细信息（类型和TTL）
async fn get_key_info(state: &Arc<WebState>, db_id: usize, key: &str) -> Option<KeyInfo> {
    let sender = state.db_manager.get_sender(db_id);
    
    // 获取键类型
    let type_frame = Frame::Array(vec![
        Frame::BulkString("TYPE".to_string()),
        Frame::BulkString(key.to_string()),
    ]);
    
    let type_command = Command::parse_from_frame(type_frame).ok()?;
    let (tx, rx) = oneshot::channel();
    sender.send(DatabaseMessage::Command { sender: tx, command: type_command }).await.ok()?;
    
    let key_type = match rx.await.ok()? {
        Frame::SimpleString(t) => t,
        _ => "unknown".to_string(),
    };
    
    // 获取TTL
    let ttl_frame = Frame::Array(vec![
        Frame::BulkString("TTL".to_string()),
        Frame::BulkString(key.to_string()),
    ]);
    
    let ttl_command = Command::parse_from_frame(ttl_frame).ok()?;
    let (tx, rx) = oneshot::channel();
    sender.send(DatabaseMessage::Command { sender: tx, command: ttl_command }).await.ok()?;
    
    let ttl = match rx.await.ok()? {
        Frame::Integer(t) => t,
        _ => -1,
    };
    
    Some(KeyInfo {
        key: key.to_string(),
        key_type,
        ttl,
    })
}

/// 获取键值
async fn get_key_value(
    State(state): State<Arc<WebState>>,
    Path(key): Path<String>,
    Query(params): Query<ListKeysQuery>,
) -> impl IntoResponse {
    let db_id = params.db.unwrap_or(0);
    
    if db_id >= state.max_databases {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "数据库索引超出范围"
            }))
        );
    }
    
    let sender = state.db_manager.get_sender(db_id);
    
    // 先获取键类型
    let type_frame = Frame::Array(vec![
        Frame::BulkString("TYPE".to_string()),
        Frame::BulkString(key.clone()),
    ]);
    
    let type_command = match Command::parse_from_frame(type_frame) {
        Ok(cmd) => cmd,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("解析TYPE命令失败: {}", e)
                }))
            );
        }
    };
    
    let (tx, rx) = oneshot::channel();
    if sender.send(DatabaseMessage::Command { sender: tx, command: type_command }).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": "发送TYPE命令失败"
            }))
        );
    }
    
    let key_type = match rx.await {
        Ok(Frame::SimpleString(t)) => t,
        _ => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "error": "键不存在"
                }))
            );
        }
    };
    
    // 根据类型获取值
    let value = match key_type.as_str() {
        "string" => get_string_value(&state, db_id, &key).await,
        "hash" => get_hash_value(&state, db_id, &key).await,
        "list" => get_list_value(&state, db_id, &key).await,
        "set" => get_set_value(&state, db_id, &key).await,
        "zset" => get_zset_value(&state, db_id, &key).await,
        _ => None,
    };
    
    if value.is_none() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": "获取键值失败"
            }))
        );
    }
    
    // 获取TTL
    let ttl_frame = Frame::Array(vec![
        Frame::BulkString("TTL".to_string()),
        Frame::BulkString(key.clone()),
    ]);
    
    let ttl_command = Command::parse_from_frame(ttl_frame).ok().unwrap();
    let (tx, rx) = oneshot::channel();
    sender.send(DatabaseMessage::Command { sender: tx, command: ttl_command }).await.ok();
    
    let ttl = match rx.await.ok() {
        Some(Frame::Integer(t)) => t,
        _ => -1,
    };
    
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": KeyValueResponse {
                key: key.clone(),
                key_type,
                value: value.unwrap(),
                ttl,
            }
        }))
    )
}

/// 获取字符串类型的值
async fn get_string_value(state: &Arc<WebState>, db_id: usize, key: &str) -> Option<serde_json::Value> {
    let sender = state.db_manager.get_sender(db_id);
    let frame = Frame::Array(vec![
        Frame::BulkString("GET".to_string()),
        Frame::BulkString(key.to_string()),
    ]);
    
    let command = Command::parse_from_frame(frame).ok()?;
    let (tx, rx) = oneshot::channel();
    sender.send(DatabaseMessage::Command { sender: tx, command }).await.ok()?;
    
    match rx.await.ok()? {
        Frame::BulkString(s) => Some(json!(s)),
        Frame::Null => None,
        _ => None,
    }
}

/// 获取哈希类型的值
async fn get_hash_value(state: &Arc<WebState>, db_id: usize, key: &str) -> Option<serde_json::Value> {
    let sender = state.db_manager.get_sender(db_id);
    let frame = Frame::Array(vec![
        Frame::BulkString("HGETALL".to_string()),
        Frame::BulkString(key.to_string()),
    ]);
    
    let command = Command::parse_from_frame(frame).ok()?;
    let (tx, rx) = oneshot::channel();
    sender.send(DatabaseMessage::Command { sender: tx, command }).await.ok()?;
    
    match rx.await.ok()? {
        Frame::Array(frames) => {
            let mut map = serde_json::Map::new();
            let mut iter = frames.into_iter();
            while let (Some(Frame::BulkString(k)), Some(Frame::BulkString(v))) = (iter.next(), iter.next()) {
                map.insert(k, json!(v));
            }
            Some(json!(map))
        }
        _ => None,
    }
}

/// 获取列表类型的值
async fn get_list_value(state: &Arc<WebState>, db_id: usize, key: &str) -> Option<serde_json::Value> {
    let sender = state.db_manager.get_sender(db_id);
    let frame = Frame::Array(vec![
        Frame::BulkString("LRANGE".to_string()),
        Frame::BulkString(key.to_string()),
        Frame::BulkString("0".to_string()),
        Frame::BulkString("-1".to_string()),
    ]);
    
    let command = Command::parse_from_frame(frame).ok()?;
    let (tx, rx) = oneshot::channel();
    sender.send(DatabaseMessage::Command { sender: tx, command }).await.ok()?;
    
    match rx.await.ok()? {
        Frame::Array(frames) => {
            let values: Vec<String> = frames.into_iter().filter_map(|f| {
                if let Frame::BulkString(s) = f {
                    Some(s)
                } else {
                    None
                }
            }).collect();
            Some(json!(values))
        }
        _ => None,
    }
}

/// 获取集合类型的值
async fn get_set_value(state: &Arc<WebState>, db_id: usize, key: &str) -> Option<serde_json::Value> {
    let sender = state.db_manager.get_sender(db_id);
    let frame = Frame::Array(vec![
        Frame::BulkString("SMEMBERS".to_string()),
        Frame::BulkString(key.to_string()),
    ]);
    
    let command = Command::parse_from_frame(frame).ok()?;
    let (tx, rx) = oneshot::channel();
    sender.send(DatabaseMessage::Command { sender: tx, command }).await.ok()?;
    
    match rx.await.ok()? {
        Frame::Array(frames) => {
            let values: Vec<String> = frames.into_iter().filter_map(|f| {
                if let Frame::BulkString(s) = f {
                    Some(s)
                } else {
                    None
                }
            }).collect();
            Some(json!(values))
        }
        _ => None,
    }
}

/// 获取有序集合类型的值
async fn get_zset_value(state: &Arc<WebState>, db_id: usize, key: &str) -> Option<serde_json::Value> {
    let sender = state.db_manager.get_sender(db_id);
    // 使用ZRANGE获取所有成员和分数
    let frame = Frame::Array(vec![
        Frame::BulkString("ZRANGE".to_string()),
        Frame::BulkString(key.to_string()),
        Frame::BulkString("0".to_string()),
        Frame::BulkString("-1".to_string()),
    ]);
    
    let command = Command::parse_from_frame(frame).ok()?;
    let (tx, rx) = oneshot::channel();
    sender.send(DatabaseMessage::Command { sender: tx, command }).await.ok()?;
    
    match rx.await.ok()? {
        Frame::Array(frames) => {
            let values: Vec<String> = frames.into_iter().filter_map(|f| {
                if let Frame::BulkString(s) = f {
                    Some(s)
                } else {
                    None
                }
            }).collect();
            Some(json!(values))
        }
        _ => None,
    }
}

/// 设置键值
async fn set_key_value(
    State(state): State<Arc<WebState>>,
    Path(key): Path<String>,
    Query(params): Query<ListKeysQuery>,
    Json(payload): Json<SetKeyRequest>,
) -> impl IntoResponse {
    let db_id = params.db.unwrap_or(0);
    
    if db_id >= state.max_databases {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "数据库索引超出范围"
            }))
        );
    }
    
    let sender = state.db_manager.get_sender(db_id);
    
    // 执行SET命令
    let frame = Frame::Array(vec![
        Frame::BulkString("SET".to_string()),
        Frame::BulkString(key.clone()),
        Frame::BulkString(payload.value),
    ]);
    
    let command = match Command::parse_from_frame(frame) {
        Ok(cmd) => cmd,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("解析SET命令失败: {}", e)
                }))
            );
        }
    };
    
    let (tx, rx) = oneshot::channel();
    if sender.send(DatabaseMessage::Command { sender: tx, command }).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": "发送SET命令失败"
            }))
        );
    }
    
    if rx.await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": "设置键值失败"
            }))
        );
    }
    
    // 如果指定了TTL，设置过期时间
    if let Some(ttl) = payload.ttl {
        let expire_frame = Frame::Array(vec![
            Frame::BulkString("EXPIRE".to_string()),
            Frame::BulkString(key.clone()),
            Frame::BulkString(ttl.to_string()),
        ]);
        
        if let Ok(expire_command) = Command::parse_from_frame(expire_frame) {
            let (tx, _rx) = oneshot::channel();
            let _ = sender.send(DatabaseMessage::Command { sender: tx, command: expire_command }).await;
        }
    }
    
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "键值设置成功"
        }))
    )
}

/// 删除键
async fn delete_key(
    State(state): State<Arc<WebState>>,
    Path(key): Path<String>,
    Query(params): Query<ListKeysQuery>,
) -> impl IntoResponse {
    let db_id = params.db.unwrap_or(0);
    
    if db_id >= state.max_databases {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "数据库索引超出范围"
            }))
        );
    }
    
    let sender = state.db_manager.get_sender(db_id);
    
    // 执行DEL命令
    let frame = Frame::Array(vec![
        Frame::BulkString("DEL".to_string()),
        Frame::BulkString(key),
    ]);
    
    let command = match Command::parse_from_frame(frame) {
        Ok(cmd) => cmd,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("解析DEL命令失败: {}", e)
                }))
            );
        }
    };
    
    let (tx, rx) = oneshot::channel();
    if sender.send(DatabaseMessage::Command { sender: tx, command }).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": "发送DEL命令失败"
            }))
        );
    }
    
    match rx.await {
        Ok(Frame::Integer(count)) if count > 0 => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "键删除成功"
            }))
        ),
        _ => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "error": "键不存在或删除失败"
            }))
        ),
    }
}

/// 更新键的TTL
async fn update_key_ttl(
    State(state): State<Arc<WebState>>,
    Path(key): Path<String>,
    Query(params): Query<ListKeysQuery>,
    Json(payload): Json<UpdateTtlRequest>,
) -> impl IntoResponse {
    let db_id = params.db.unwrap_or(0);
    
    if db_id >= state.max_databases {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "数据库索引超出范围"
            }))
        );
    }
    
    let sender = state.db_manager.get_sender(db_id);
    
    // 执行EXPIRE命令
    let frame = Frame::Array(vec![
        Frame::BulkString("EXPIRE".to_string()),
        Frame::BulkString(key),
        Frame::BulkString(payload.ttl.to_string()),
    ]);
    
    let command = match Command::parse_from_frame(frame) {
        Ok(cmd) => cmd,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("解析EXPIRE命令失败: {}", e)
                }))
            );
        }
    };
    
    let (tx, rx) = oneshot::channel();
    if sender.send(DatabaseMessage::Command { sender: tx, command }).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": "发送EXPIRE命令失败"
            }))
        );
    }
    
    match rx.await {
        Ok(Frame::Integer(1)) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "TTL更新成功"
            }))
        ),
        _ => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "error": "键不存在或TTL更新失败"
            }))
        ),
    }
}
