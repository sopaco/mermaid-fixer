use serde::{Deserialize, Serialize};

use crate::config::Config;

pub struct AiFixer {
    api_key: String,
    model: String,
    prompt_template: String,
    base_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
struct FixResponse {
    fixed_code: String,
    explanation: String,
    changes: Option<Vec<ChangeDetail>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChangeDetail {
    #[serde(rename = "type")]
    change_type: String,
    original: String,
    fixed: String,
    reason: String,
}

impl AiFixer {
    pub async fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        // 获取API密钥
        let api_key = config.llm.api_key.clone()
            .ok_or("未找到LLM API密钥，请通过参数或配置文件指定")?;

        // 读取prompt模板
        let prompt_template = include_str!("prompt.tpl").to_owned();

        let base_url = config.llm.base_url.clone()
            .unwrap_or_else(|| "https://api.mistral.ai/v1".to_string());

        Ok(Self {
            api_key,
            model: config.llm.model.clone(),
            prompt_template,
            base_url,
        })
    }

    /// 修复mermaid代码
    pub async fn fix_mermaid(&self, mermaid_code: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 构建提示词
        let prompt = self.build_prompt(mermaid_code);
        
        // 构建请求
        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "user".to_string(),
                    content: prompt,
                }
            ],
            max_tokens: Some(65536),
            temperature: Some(0.1),
        };

        // 发送HTTP请求
        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("LLM API请求失败: {}", error_text).into());
        }

        let openai_response: OpenAIResponse = response.json().await?;
        
        if openai_response.choices.is_empty() {
            return Err("LLM API返回空响应".into());
        }

        let content = &openai_response.choices[0].message.content;
        
        // 解析响应，提取修复后的代码
        let fixed_code = self.extract_fixed_code(content)?;
        
        Ok(fixed_code)
    }

    /// 构建提示词
    fn build_prompt(&self, mermaid_code: &str) -> String {
        self.prompt_template.replace("{{MERMAID_CODE}}", mermaid_code)
    }

    /// 从LLM响应中提取修复后的代码
    fn extract_fixed_code(&self, response: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 清理响应内容，移除可能的markdown代码块包装
        let cleaned_response = self.clean_response(response);
        
        // 尝试解析JSON格式的响应
        if let Ok(fix_response) = serde_json::from_str::<FixResponse>(&cleaned_response) {
            println!("         📋 修复说明: {}", fix_response.explanation);
            if let Some(changes) = &fix_response.changes {
                for (i, change) in changes.iter().enumerate() {
                    println!("         🔧 修改 {}: {} -> {}", 
                        i + 1, change.change_type, change.reason);
                }
            }
            return Ok(fix_response.fixed_code);
        }

        // 如果不是JSON格式，尝试提取markdown代码块
        if let Some(code) = self.extract_code_block(response) {
            return Ok(code);
        }

        // 如果都失败了，返回原始响应（去除前后空白）
        Ok(response.trim().to_string())
    }

    /// 清理响应内容，移除可能的markdown包装
    fn clean_response(&self, response: &str) -> String {
        let response = response.trim();
        
        // 如果响应被```json包装，提取其中的JSON内容
        if response.starts_with("```json") && response.ends_with("```") {
            let lines: Vec<&str> = response.lines().collect();
            if lines.len() > 2 {
                return lines[1..lines.len()-1].join("\n");
            }
        }
        
        // 如果响应被```包装，提取其中的内容
        if response.starts_with("```") && response.ends_with("```") {
            let lines: Vec<&str> = response.lines().collect();
            if lines.len() > 2 {
                return lines[1..lines.len()-1].join("\n");
            }
        }
        
        response.to_string()
    }

    /// 从markdown格式的响应中提取代码块
    fn extract_code_block(&self, response: &str) -> Option<String> {
        let lines: Vec<&str> = response.lines().collect();
        let mut in_code_block = false;
        let mut code_lines = Vec::new();
        
        for line in lines {
            if line.trim().starts_with("```mermaid") {
                in_code_block = true;
                continue;
            }
            
            if line.trim() == "```" && in_code_block {
                break;
            }
            
            if in_code_block {
                code_lines.push(line);
            }
        }
        
        if !code_lines.is_empty() {
            Some(code_lines.join("\n"))
        } else {
            None
        }
    }
}