use crate::canvas::Canvas;

/// 2D 渲染器
///
/// 管理渲染队列，按层级排序后批量渲染。
pub struct Renderer {
    canvas: Canvas,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            canvas: Canvas::new(width, height),
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
}
