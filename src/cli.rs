use clap::Parser;
use std::path::PathBuf;

use crate::config::Config;

/// Mermaid Fixer - 基于Rust和AI的markdown文档修复器
#[derive(Parser, Debug)]
#[command(name = "mermaid-fixer")]
#[command(about = "基于Rust和AI的markdown文档修复器，用于扫描和修复markdown文件中的mermaid图表语法错误")]
#[command(author = "jiangmeng03")]
#[command(version)]
pub struct Args {
    /// 要扫描的目录路径
    #[arg(short, long, value_name = "DIR")]
    pub directory: PathBuf,

    /// 配置文件路径
    #[arg(short, long, value_name = "FILE", default_value = "config.toml")]
    pub config: PathBuf,

    /// 只检测问题，不进行修复
    #[arg(long)]
    pub dry_run: bool,

    /// 启用详细日志输出
    #[arg(short, long)]
    pub verbose: bool,

    /// LLM提供商 (openai, mistral, deepseek等)
    #[arg(long)]
    pub llm_provider: Option<String>,

    /// LLM模型名称
    #[arg(long)]
    pub llm_model: Option<String>,

    /// LLM API密钥
    #[arg(long)]
    pub llm_api_key: Option<String>,

    /// LLM API基础URL
    #[arg(long)]
    pub llm_base_url: Option<String>,

    /// 最大token数
    #[arg(long)]
    pub max_tokens: Option<u32>,

    /// 温度参数 (0.0-1.0)
    #[arg(long)]
    pub temperature: Option<f32>,

    /// Mermaid验证超时时间（秒）
    #[arg(long)]
    pub timeout_seconds: Option<u64>,

    /// 最大重试次数
    #[arg(long)]
    pub max_retries: Option<u32>,
}

impl Args {
    /// 将CLI参数转换为配置
    pub fn to_config(self) -> Result<Config, Box<dyn std::error::Error>> {
        let mut config = if self.config.exists() {
            Config::load(&self.config)?
        } else {
            Config::default()
        };

        // 覆盖LLM配置
        if let Some(provider) = self.llm_provider {
            config.llm.provider = provider;
        }

        if let Some(model) = self.llm_model {
            config.llm.model = model;
        }

        if let Some(api_key) = self.llm_api_key {
            config.llm.api_key = Some(api_key);
        }

        if let Some(base_url) = self.llm_base_url {
            config.llm.base_url = Some(base_url);
        }

        if let Some(max_tokens) = self.max_tokens {
            config.llm.max_tokens = Some(max_tokens);
        }

        if let Some(temperature) = self.temperature {
            config.llm.temperature = Some(temperature);
        }

        // 覆盖Mermaid配置
        if let Some(timeout) = self.timeout_seconds {
            config.mermaid.timeout_seconds = Some(timeout);
        }

        if let Some(retries) = self.max_retries {
            config.mermaid.max_retries = Some(retries);
        }

        Ok(config)
    }
}