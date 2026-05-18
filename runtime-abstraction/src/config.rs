use serde::{Deserialize, Serialize};

/// 运行时配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// 画布宽度
    pub width: u32,
    /// 画布高度
    pub height: u32,
    /// 目标帧率
    pub target_fps: u32,
    /// 背景色 (RGBA)
    pub background_color: [u8; 4],
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            target_fps: 60,
            background_color: [0, 0, 0, 255],
        }
    }
}
