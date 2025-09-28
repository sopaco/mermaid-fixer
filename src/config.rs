use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub llm: LlmConfig,
    pub mermaid: MermaidConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LlmConfig {
    pub provider: String,
    pub model: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MermaidConfig {
    pub timeout_seconds: Option<u64>,
    pub max_retries: Option<u32>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            llm: LlmConfig {
                provider: "mistral".to_string(),
                model: "mistral-small-latest".to_string(),
                api_key: None,
                base_url: None,
                max_tokens: Some(65536),
                temperature: Some(0.1),
            },
            mermaid: MermaidConfig {
                timeout_seconds: Some(120),
                max_retries: Some(3),
            },
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref();
        
        if !path.exists() {
            // 如果配置文件不存在，创建默认配置文件
            let default_config = Self::default();
            let toml_content = toml::to_string_pretty(&default_config)?;
            fs::write(path, toml_content)?;
            println!("📝 已创建默认配置文件: {}", path.display());
            return Ok(default_config);
        }

        let content = fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(&content)?;
        
        // 从环境变量中读取API密钥（如果配置文件中没有设置）
        if config.llm.api_key.is_none() {
            config.llm.api_key = std::env::var("LITHO_LLM_API_KEY").ok();
        }

        Ok(config)
    }
}