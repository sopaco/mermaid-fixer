use std::fs;
use std::path::{Path, PathBuf};

pub struct MarkdownScanner {
    // 可以添加配置选项，比如忽略的目录等
}

impl MarkdownScanner {
    pub fn new() -> Self {
        Self {}
    }

    /// 扫描指定目录下的所有markdown文件
    pub fn scan_directory<P: AsRef<Path>>(&self, dir: P) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let mut markdown_files = Vec::new();
        self.scan_recursive(dir.as_ref(), &mut markdown_files)?;
        Ok(markdown_files)
    }

    /// 递归扫描目录
    fn scan_recursive(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
        if !dir.is_dir() {
            return Err(format!("路径不是目录: {}", dir.display()).into());
        }

        let entries = fs::read_dir(dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // 跳过一些常见的不需要扫描的目录
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if self.should_skip_directory(dir_name) {
                        continue;
                    }
                }
                
                // 递归扫描子目录
                self.scan_recursive(&path, files)?;
            } else if path.is_file() {
                // 检查是否是markdown文件
                if self.is_markdown_file(&path) {
                    files.push(path);
                }
            }
        }

        Ok(())
    }

    /// 判断是否是markdown文件
    fn is_markdown_file(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            matches!(extension.to_lowercase().as_str(), "md" | "markdown")
        } else {
            false
        }
    }

    /// 判断是否应该跳过某个目录
    fn should_skip_directory(&self, dir_name: &str) -> bool {
        matches!(
            dir_name,
            ".git" | ".svn" | ".hg" | 
            "node_modules" | "target" | "build" | "dist" |
            ".vscode" | ".idea" | ".vs" |
            "__pycache__" | ".pytest_cache" |
            "vendor" | "deps"
        )
    }
}
