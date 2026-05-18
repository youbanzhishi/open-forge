use serde::{Deserialize, Serialize};

/// 游戏事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    /// 键盘按下
    KeyDown { key: String },
    /// 键盘释放
    KeyUp { key: String },
    /// 鼠标按下
    MouseDown { x: f64, y: f64, button: MouseButton },
    /// 鼠标释放
    MouseUp { x: f64, y: f64, button: MouseButton },
    /// 鼠标移动
    MouseMove { x: f64, y: f64 },
    /// 触摸开始
    TouchStart { id: u32, x: f64, y: f64 },
    /// 触摸结束
    TouchEnd { id: u32 },
    /// 窗口大小变化
    Resize { width: u32, height: u32 },
    /// 自定义事件
    Custom { name: String, data: serde_json::Value },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}
