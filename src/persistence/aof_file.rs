use std::{fs, path::PathBuf, time::Duration};

use anyhow::Result;
use tokio::{fs::OpenOptions, io::AsyncWriteExt, sync::mpsc::{self, Receiver, Sender}, time::{interval, Interval}};

use crate::frame::Frame;

#[derive(Debug, Clone)]
pub enum SyncStrategy {
    Always,
    EverySec,
    No,
}

impl SyncStrategy {
    pub fn from_str(strategy: &str) -> Self {
        match strategy {
            "always" => SyncStrategy::Always,
            "everysec" => SyncStrategy::EverySec,
            "no" => SyncStrategy::No,
            _ => SyncStrategy::Always, // 默认为 always
        }
    }
}

pub struct AofFile {
    sender: Sender<(usize, Frame)>,
    file_path: PathBuf
}

impl AofFile {
    
    /// 创建 AOF 处理实例
    pub fn new(file_path: PathBuf, sync_strategy: SyncStrategy) -> Self {
        let (sender, receiver) = mpsc::channel(1024);
        let aof_file = AofFile {
            sender,
            file_path: file_path.clone()
        };
        tokio::spawn(Self::persist_loop(file_path, receiver, sync_strategy));
        aof_file
    }

    /// 获取 AOF 发送通道
    pub fn get_sender(&self) -> Sender<(usize, Frame)> {
        self.sender.clone()
    }

    pub async fn read_all_frames(&self) -> Result<Vec<Frame>> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let content = tokio::fs::read(&self.file_path).await?;
        let mut frames = Vec::new();
        let mut start = 0;
        let separator = b"\r\n\r\n";

        // 遍历内容查找分隔符
        while let Some(pos) =&content[start..].windows(separator.len()).position(|window| window == separator) {
            let end = start + pos;
            let frame_data = &content[start..end + separator.len() / 2];
            if !frame_data.is_empty() {
                if let Ok(frame) = Frame::parse_from_bytes(frame_data) {
                    frames.push(frame);
                }
            }
            // 跳过分隔符（4字节）
            start = end + separator.len();
        }
        Ok(frames)
    }
    
    /// 后台 AOF 写入任务
    pub async fn persist_loop(file_path: PathBuf, mut receiver: Receiver<(usize, Frame)>, sync_strategy: SyncStrategy) {

        // 确保目录存在
        if let Some(parent) = file_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                log::error!("Failed to create AOF directory: {}", e);
                return;  // 目录创建失败时退出任务
            }
        }

        let mut open_options = OpenOptions::new();
        open_options.create(true);
        open_options.append(true);

        // 确保文件存在
        let mut file = match open_options.open(&file_path).await {
            Ok(file) => file,
            Err(e) => {
                log::error!("Failed to open AOF file: {}", e);
                return;  // 文件打开失败时退出任务
            }
        };

        let mut current_db_index = 0; // 跟踪数据库索引
        
        // 为 everysec 策略创建定时器
        let mut interval_timer: Option<Interval> = match sync_strategy {
            SyncStrategy::EverySec => {
                let mut interval = interval(Duration::from_secs(1));
                interval.tick().await; // 消耗第一次 tick
                Some(interval)
            },
            _ => None,
        };
        
        // 用于跟踪是否需要 flush
        let mut need_flush = false;
        
        loop {
            // 对于 everysec 策略，我们既要处理消息也要处理定时器
            if let SyncStrategy::EverySec = sync_strategy {
                tokio::select! {
                    // 处理接收的消息
                    msg = receiver.recv() => {
                        if let Some((idx, frame)) = msg {
                            if let Err(e) = Self::write_frame(&mut file, &mut current_db_index, idx, &frame, &mut need_flush).await {
                                log::error!("Failed to write command to AOF file: {}", e);
                                continue;
                            }
                        } else {
                            // 通道关闭，退出循环
                            break;
                        }
                    },
                    // 处理定时 flush
                    _ = interval_timer.as_mut().unwrap().tick() => {
                        if need_flush {
                            if let Err(e) = file.flush().await {
                                log::error!("Failed to flush AOF file: {}", e);
                            } else {
                                need_flush = false;
                            }
                        }
                    }
                }
            } else {
                // 对于 always 和 no 策略，只需要处理消息
                if let Some((idx, frame)) = receiver.recv().await {
                    if let Err(e) = Self::write_frame(&mut file, &mut current_db_index, idx, &frame, &mut need_flush).await {
                        log::error!("Failed to write command to AOF file: {}", e);
                        continue;
                    }
                    
                    // 根据策略决定是否 flush
                    match sync_strategy {
                        SyncStrategy::Always => {
                            if let Err(e) = file.flush().await {
                                log::error!("Failed to flush AOF file: {}", e);
                            }
                        },
                        SyncStrategy::EverySec => unreachable!(), 
                        SyncStrategy::No => {},
                    }
                } else {
                    // 通道关闭，退出循环
                    break;
                }
            }
        }
    }
    
    /// 写入帧数据的辅助函数
    async fn write_frame(
        file: &mut tokio::fs::File,
        current_db_index: &mut usize,
        idx: usize,
        frame: &Frame,
        need_flush: &mut bool,
    ) -> Result<()> {
        if idx != *current_db_index {
            let select_frame = Frame::Array(vec![
                Frame::BulkString("SELECT".to_string()),
                Frame::BulkString(idx.to_string()),
            ]);

            file.write_all(&select_frame.as_bytes()).await?;
            file.write_all(b"\r\n").await?;

            *current_db_index = idx;
            *need_flush = true;
        }
       
        file.write_all(&frame.as_bytes()).await?;
        file.write_all(b"\r\n").await?;
        *need_flush = true;
        
        Ok(())
    }
}