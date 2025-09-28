use std::path::PathBuf;

use crate::config::Config;
use crate::markdown_scanner::MarkdownScanner;
use crate::mermaid_validator::MermaidValidator;
use crate::ai_fixer::AiFixer;
use crate::utils::extract_mermaid_blocks;

pub struct MermaidProcessor {
    scanner: MarkdownScanner,
    validator: MermaidValidator,
    ai_fixer: Option<AiFixer>,
    verbose: bool,
}

pub struct ProcessResult {
    pub total_files: usize,
    pub total_mermaid_blocks: usize,
    pub invalid_blocks: usize,
    pub fixed_blocks: usize,
}

impl MermaidProcessor {
    pub async fn new(config: &Config, dry_run: bool, verbose: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let scanner = MarkdownScanner::new();
        
        let validator = MermaidValidator::with_config(config.mermaid.timeout_seconds)?;
        
        let ai_fixer = if dry_run {
            None
        } else {
            Some(AiFixer::new(config).await?)
        };

        Ok(Self {
            scanner,
            validator,
            ai_fixer,
            verbose,
        })
    }

    pub async fn process_directory(&self, directory: PathBuf, dry_run: bool) -> Result<ProcessResult, Box<dyn std::error::Error>> {
        if self.verbose {
            println!("🚀 开始扫描目录: {}", directory.display());
        }
        
        // 扫描markdown文件
        let markdown_files = self.scanner.scan_directory(&directory)?;
        
        if self.verbose {
            println!("📄 找到 {} 个markdown文件", markdown_files.len());
        }

        let mut total_mermaid_blocks = 0;
        let mut invalid_blocks = 0;
        let mut fixed_blocks = 0;

        // 处理每个markdown文件
        for file_path in &markdown_files {
            if self.verbose {
                println!("\n📝 处理文件: {}", file_path.display());
            }
            
            let result = self.process_file(file_path, dry_run).await?;
            
            total_mermaid_blocks += result.total_blocks;
            invalid_blocks += result.invalid_blocks;
            fixed_blocks += result.fixed_blocks;
        }

        Ok(ProcessResult {
            total_files: markdown_files.len(),
            total_mermaid_blocks,
            invalid_blocks,
            fixed_blocks,
        })
    }

    async fn process_file(&self, file_path: &PathBuf, dry_run: bool) -> Result<FileProcessResult, Box<dyn std::error::Error>> {
        // 读取文件内容
        let content = std::fs::read_to_string(file_path)?;

        // 提取mermaid代码块
        let mermaid_blocks = extract_mermaid_blocks(&content);
        
        if mermaid_blocks.is_empty() {
            if self.verbose {
                println!("   ℹ️  未找到mermaid代码块");
            }
            return Ok(FileProcessResult::default());
        }

        if self.verbose {
            println!("   🔍 找到 {} 个mermaid代码块", mermaid_blocks.len());
        }

        let mut invalid_blocks = 0;
        let mut fixed_blocks = 0;
        let mut file_modified = false;
        let mut new_content = content.clone();

        // 验证每个mermaid代码块
        for (index, (_start_pos, _end_pos, mermaid_code)) in mermaid_blocks.iter().enumerate() {
            if self.verbose {
                println!("      📊 验证代码块 {}/{}", index + 1, mermaid_blocks.len());
            }
            
            let validator = MermaidValidator::with_config(None)?;
            
            match validator.validate(mermaid_code) {
                Ok(_) => {
                    if self.verbose {
                        println!("         ✅ 代码块有效");
                    }
                }
                Err(e) => {
                    if self.verbose {
                        println!("         ❌ 代码块无效: {}", e);
                    }
                    invalid_blocks += 1;

                    if !dry_run {
                        if let Some(ai_fixer) = &self.ai_fixer {
                            match ai_fixer.fix_mermaid(mermaid_code).await {
                                Ok(fixed_code) => {
                                    // 验证修复后的代码
                                    match validator.validate(&fixed_code) {
                                        Ok(_) => {
                                            if self.verbose {
                                                println!("         🔧 修复成功");
                                            }
                                            // 替换原始代码
                                            new_content = new_content.replace(mermaid_code, &fixed_code);
                                            file_modified = true;
                                            fixed_blocks += 1;
                                        }
                                        Err(validation_error) => {
                                            if self.verbose {
                                                println!("         ⚠️  修复后的代码仍然无效: {}", validation_error);
                                            }
                                        }
                                    }
                                }
                                Err(fix_error) => {
                                    if self.verbose {
                                        println!("         ⚠️  AI修复失败: {}", fix_error);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // 如果文件被修改，写回文件
        if file_modified {
            std::fs::write(file_path, new_content)?;
            if self.verbose {
                println!("   💾 文件已更新");
            }
        }

        Ok(FileProcessResult {
            total_blocks: mermaid_blocks.len(),
            invalid_blocks,
            fixed_blocks,
        })
    }
}

#[derive(Default)]
struct FileProcessResult {
    total_blocks: usize,
    invalid_blocks: usize,
    fixed_blocks: usize,
}