use crate::canvas::Canvas;

/// 2D 渲染器
///
/// 管理渲染队列，按层级排序后批量渲染。
pub struct Renderer {
    canvas: Canvas,
    width: u32,
    height: u32,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            canvas: Canvas::new(width, height),
            width,
            height,
        }
    }

    /// 开始新帧
    pub fn begin_frame(&self, background: [u8; 4]) {
        self.canvas.clear(background);
    }

    /// 提交帧（交换缓冲区）
    pub fn end_frame(&self) {
        // TODO: 提交到显示
    }

    /// 获取画布引用
    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }
    
    /// 绘制精灵
    pub fn draw_sprite(
        &self,
        _image: &str,
        x: f64,
        y: f64,
        rotation: f64,
        scale_x: f64,
        scale_y: f64,
    ) {
        // TODO: 加载并绘制图片
        // 临时用矩形占位
        let _ = (x, y, rotation, scale_x, scale_y);
    }
    
    /// 绘制矩形
    pub fn draw_rect(&self, x: f64, y: f64, width: f64, height: f64, color: &str) {
        let c = Self::parse_color(color);
        self.canvas.fill_rect(x as i32, y as i32, width as u32, height as u32, c);
    }
    
    /// 绘制圆形
    pub fn draw_circle(&self, x: f64, y: f64, radius: f64, color: &str) {
        let c = Self::parse_color(color);
        self.canvas.fill_circle(x as i32, y as i32, radius as u32, c);
    }
    
    /// 绘制线条
    pub fn draw_line(&self, x1: f64, y1: f64, x2: f64, y2: f64, color: &str, width: f64) {
        let c = Self::parse_color(color);
        self.canvas.draw_line(
            x1 as i32, y1 as i32,
            x2 as i32, y2 as i32,
            c,
            width as u32,
        );
    }
    
    /// 绘制文字
    pub fn draw_text(&self, text: &str, x: f64, y: f64, size: f32, color: &str) {
        let c = Self::parse_color(color);
        // TODO: 实际渲染文字
        // 临时用矩形占位表示文字位置
        let w = (text.len() as f32 * size * 0.6) as u32;
        let h = size as u32;
        self.canvas.fill_rect(x as i32, y as i32, w, h, c);
    }
    
    /// 绘制图片
    pub fn draw_image(&self, image_data: &[u8], x: f64, y: f64, width: f64, height: f64) {
        let _ = (image_data, x, y, width, height);
        // TODO: 解码并绘制图片
    }
    
    /// 设置裁剪区域
    pub fn set_clip(&self, x: i32, y: i32, width: u32, height: u32) {
        // TODO: 设置裁剪
        let _ = (x, y, width, height);
    }
    
    /// 清除裁剪区域
    pub fn clear_clip(&self) {
        // TODO: 清除裁剪
    }
    
    /// 解析颜色字符串 (#RRGGBBAA 或 #RRGGBB)
    fn parse_color(color: &str) -> [u8; 4] {
        let hex = color.trim_start_matches('#');
        match hex.len() {
            6 => [
                u8::from_str_radix(&hex[0..2], 16).unwrap_or(255),
                u8::from_str_radix(&hex[2..4], 16).unwrap_or(255),
                u8::from_str_radix(&hex[4..6], 16).unwrap_or(255),
                255,
            ],
            8 => [
                u8::from_str_radix(&hex[0..2], 16).unwrap_or(255),
                u8::from_str_radix(&hex[2..4], 16).unwrap_or(255),
                u8::from_str_radix(&hex[4..6], 16).unwrap_or(255),
                u8::from_str_radix(&hex[6..8], 16).unwrap_or(255),
            ],
            _ => [255, 255, 255, 255],
        }
    }
}
