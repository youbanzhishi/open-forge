/// 画布抽象
///
/// 封装 Canvas 2D / WebGL 渲染上下文。
/// Phase 1: 纯 2D Canvas API
/// Phase 2: 可选 WebGL 加速
pub struct Canvas {
    width: u32,
    height: u32,
    clip: Option<(i32, i32, u32, u32)>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self { 
            width, 
            height,
            clip: None,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// 清空画布
    pub fn clear(&self, _color: [u8; 4]) {
        // TODO: 实际清空逻辑（Web Canvas API 调用）
    }

    /// 绘制精灵
    pub fn draw_sprite(
        &self,
        _asset_id: &str,
        _x: f64,
        _y: f64,
        _width: f64,
        _height: f64,
        _rotation: f64,
    ) {
        // TODO: 实际绘制逻辑
    }

    /// 绘制文字
    pub fn draw_text(&self, _text: &str, _x: f64, _y: f64, _font: &str, _color: [u8; 4]) {
        // TODO: 实际绘制逻辑
    }
    
    /// 填充矩形
    pub fn fill_rect(&self, x: i32, y: i32, width: u32, height: u32, color: [u8; 4]) {
        let _ = (x, y, width, height, color);
        // TODO: 实现
    }
    
    /// 填充圆形
    pub fn fill_circle(&self, x: i32, y: i32, radius: u32, color: [u8; 4]) {
        let _ = (x, y, radius, color);
        // TODO: 实现
    }
    
    /// 绘制线条
    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: [u8; 4], width: u32) {
        let _ = (x1, y1, x2, y2, color, width);
        // TODO: 实现
    }
    
    /// 设置裁剪区域
    pub fn set_clip(&mut self, x: i32, y: i32, width: u32, height: u32) {
        self.clip = Some((x, y, width, height));
    }
    
    /// 清除裁剪区域
    pub fn clear_clip(&mut self) {
        self.clip = None;
    }
}
