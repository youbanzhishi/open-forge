/// 画布抽象
///
/// 封装 Canvas 2D / WebGL 渲染上下文。
/// Phase 1: 纯 2D Canvas API
/// Phase 2: 可选 WebGL 加速
pub struct Canvas {
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
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
}
