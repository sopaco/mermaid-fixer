use clap::Parser;
use std::process;

mod cli;
mod config;
mod markdown_scanner;
mod mermaid_validator;
mod ai_fixer;
mod processor;
mod utils;

use cli::Args;
use processor::MermaidProcessor;
use utils::print_statistics;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    // 提取需要的值
    let directory = args.directory.clone();
    let dry_run = args.dry_run;
    let verbose = args.verbose;
    
    // 解析配置
    let config = match args.to_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("❌ 配置错误: {}", e);
            process::exit(1);
        }
    };

    // 创建处理器
    let processor = match MermaidProcessor::new(&config, dry_run, verbose).await {
        Ok(processor) => processor,
        Err(e) => {
            eprintln!("❌ 初始化失败: {}", e);
            process::exit(1);
        }
    };

    // 处理目录
    let result = match processor.process_directory(directory, dry_run).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("❌ 处理失败: {}", e);
            process::exit(1);
        }
    };

    // 输出统计信息
    print_statistics(&result, dry_run);
}