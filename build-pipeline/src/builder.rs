use crate::error::BuildError;
use crate::manifest::{BuildManifest, BuildResult, BuildTarget};

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

        // TODO: 实际构建逻辑
        // 1. 编译场景 YAML → JS
        // 2. 打包运行时
        // 3. 生成 index.html

        Ok(BuildResult {
            build_id: manifest.build_id.clone(),
            success: true,
            output_path: Some(output_path),
            output_size: Some(0),
            error: None,
            duration_ms: 0,
        })
    }
}
