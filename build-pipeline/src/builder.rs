use crate::error::BuildError;
use crate::manifest::{BuildManifest, BuildResult, BuildTarget};
use std::path::Path;

/// 构建管线
pub struct BuildPipeline {
    output_dir: String,
}

impl BuildPipeline {
    pub fn new(output_dir: &str) -> Self {
        Self {
            output_dir: output_dir.to_string(),
        }
    }

    /// 执行构建
    pub async fn build(&self, manifest: &BuildManifest) -> Result<BuildResult, BuildError> {
        let start = std::time::Instant::now();

        match manifest.target {
            BuildTarget::Web => self.build_web(manifest).await,
            _ => Err(BuildError::UnsupportedTarget(format!("{:?}", manifest.target))),
        }
        .map(|mut result| {
            result.duration_ms = start.elapsed().as_millis() as u64;
            result
        })
    }

    /// 构建 HTML5 Web 包
    async fn build_web(&self, manifest: &BuildManifest) -> Result<BuildResult, BuildError> {
        let output_path = format!("{}/{}", self.output_dir, manifest.build_id);
        
        // 创建输出目录
        std::fs::create_dir_all(&output_path).map_err(|e| {
            BuildError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!(
                "Failed to create output directory: {}", e
            )))
        })?;

        // 生成 index.html
        let html = self.generate_html(manifest);
        std::fs::write(format!("{}/index.html", output_path), html).map_err(|e| {
            BuildError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!(
                "Failed to write index.html: {}", e
            )))
        })?;

        // 生成 game.js (游戏逻辑)
        let js = self.generate_game_js(manifest);
        std::fs::write(format!("{}/game.js", output_path), js).map_err(|e| {
            BuildError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!(
                "Failed to write game.js: {}", e
            )))
        })?;

        // 计算输出大小
        let output_size = std::fs::read_dir(&output_path)
            .map(|entries| {
                entries.filter_map(|e| e.ok())
                    .filter_map(|e| e.metadata().ok())
                    .map(|m| m.len())
                    .sum()
            })
            .unwrap_or(0);

        tracing::info!("Web build completed: {} ({} bytes)", output_path, output_size);

        Ok(BuildResult {
            build_id: manifest.build_id.clone(),
            success: true,
            output_path: Some(output_path),
            output_size: Some(output_size),
            error: None,
            duration_ms: 0,
        })
    }

    /// 生成 HTML 文件
    fn generate_html(&self, manifest: &BuildManifest) -> String {
        let title = &manifest.project_name;
        let width = manifest.config.get("width").and_then(|v| v.as_u64()).unwrap_or(800);
        let height = manifest.config.get("height").and_then(|v| v.as_u64()).unwrap_or(600);
        
        format!(r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            background: #000;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            font-family: Arial, sans-serif;
        }}
        #game-canvas {{
            background: #1a1a2e;
            border: 2px solid #e94560;
        }}
    </style>
</head>
<body>
    <canvas id="game-canvas" width="{width}" height="{height}"></canvas>
    <script src="game.js"></script>
</body>
</html>
"#, title = title, width = width, height = height)
    }

    /// 生成游戏逻辑 JS
    fn generate_game_js(&self, manifest: &BuildManifest) -> String {
        let scenes_json = serde_json::to_string(&manifest.scenes).unwrap_or_default();
        
        format!(r#"(function() {{
    // OpenForge Runtime
    const canvas = document.getElementById('game-canvas');
    const ctx = canvas.getContext('2d');
    
    // 游戏状态
    let currentScene = null;
    let entities = [];
    let lastTime = 0;
    let frameCount = 0;
    let fps = 0;
    
    // 场景数据
    const scenes = {scenes_json};
    
    // 初始化
    function init() {{
        if (scenes.length > 0) {{
            loadScene(scenes[0]);
        }}
        requestAnimationFrame(gameLoop);
    }}
    
    // 加载场景
    function loadScene(scene) {{
        currentScene = scene;
        entities = scene.entities || [];
        console.log('Loaded scene:', scene.name);
    }}
    
    // 游戏循环
    function gameLoop(timestamp) {{
        // FPS 计算
        frameCount++;
        if (timestamp - lastTime >= 1000) {{
            fps = frameCount;
            frameCount = 0;
            lastTime = timestamp;
        }}
        
        // 更新
        update();
        
        // 渲染
        render();
        
        requestAnimationFrame(gameLoop);
    }}
    
    // 更新
    function update() {{
        entities.forEach(entity => {{
            if (entity.components && entity.components.movement) {{
                const m = entity.components.movement;
                if (m.vx) entity.x += m.vx;
                if (m.vy) entity.y += m.vy;
            }}
        }});
    }}
    
    // 渲染
    function render() {{
        // 清空画布
        ctx.fillStyle = '#1a1a2e';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        
        // 绘制实体
        entities.forEach(entity => {{
            if (!entity.visible) return;
            
            // 绘制矩形
            if (entity.components && entity.components.rect) {{
                const rect = entity.components.rect;
                ctx.fillStyle = rect.color || '#e94560';
                ctx.fillRect(entity.x, entity.y, rect.width || 50, rect.height || 50);
            }}
            
            // 绘制精灵
            if (entity.components && entity.components.sprite) {{
                const sprite = entity.components.sprite;
                ctx.fillStyle = sprite.color || '#4caf50';
                ctx.beginPath();
                ctx.arc(entity.x, entity.y, sprite.radius || 20, 0, Math.PI * 2);
                ctx.fill();
            }}
            
            // 绘制文字
            if (entity.components && entity.components.text) {{
                const text = entity.components.text;
                ctx.fillStyle = text.color || '#fff';
                ctx.font = (text.size || 16) + 'px Arial';
                ctx.fillText(text.content || '', entity.x, entity.y);
            }}
        }});
        
        // 绘制标题
        if (currentScene) {{
            ctx.fillStyle = '#e94560';
            ctx.font = '24px Arial';
            ctx.fillText(currentScene.name || 'OpenForge Game', 20, 40);
            ctx.fillStyle = '#888';
            ctx.font = '14px Arial';
            ctx.fillText('FPS: ' + fps, 20, 65);
        }}
    }}
    
    // 启动
    init();
}})();
"#, scenes_json = scenes_json)
    }
}
