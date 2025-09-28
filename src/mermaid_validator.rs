use mermaid_rs::Mermaid;
use std::time::Duration;

pub struct MermaidValidator {
    mermaid: Mermaid,
    timeout: Duration,
}

impl MermaidValidator {

    pub fn with_config(timeout_seconds: Option<u64>) -> Result<Self, Box<dyn std::error::Error>> {
        let mermaid = Mermaid::new()
            .map_err(|e| format!("初始化Mermaid实例失败: {}", e))?;
        
        let timeout = Duration::from_secs(timeout_seconds.unwrap_or(30));
        
        Ok(Self {
            mermaid,
            timeout,
        })
    }

    /// 验证mermaid代码是否有效
    pub fn validate(&self, mermaid_code: &str) -> Result<(), MermaidValidationError> {
        if mermaid_code.trim().is_empty() {
            return Err(MermaidValidationError::EmptyCode);
        }

        // 预处理代码：移除可能的前后空白和注释
        let cleaned_code = self.preprocess_code(mermaid_code);

        // 使用mermaid-rs进行验证
        match self.mermaid.render(&cleaned_code) {
            Ok(_) => Ok(()),
            Err(e) => {
                // 分析错误类型
                let error_message = e.to_string();
                let error_type = self.classify_error(&error_message);
                Err(MermaidValidationError::RenderError {
                    message: error_message,
                    error_type,
                    original_code: mermaid_code.to_string(),
                })
            }
        }
    }

    /// 预处理mermaid代码
    fn preprocess_code(&self, code: &str) -> String {
        code.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with("%%"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// 分类错误类型
    fn classify_error(&self, error_message: &str) -> MermaidErrorType {
        let error_lower = error_message.to_lowercase();
        
        if error_lower.contains("syntax") || error_lower.contains("parse") {
            MermaidErrorType::SyntaxError
        } else if error_lower.contains("node") || error_lower.contains("vertex") {
            MermaidErrorType::NodeError
        } else if error_lower.contains("edge") || error_lower.contains("arrow") || error_lower.contains("link") {
            MermaidErrorType::EdgeError
        } else if error_lower.contains("graph") || error_lower.contains("diagram") {
            MermaidErrorType::GraphStructureError
        } else if error_lower.contains("style") || error_lower.contains("class") {
            MermaidErrorType::StyleError
        } else {
            MermaidErrorType::Unknown
        }
    }
}

#[derive(Debug, Clone)]
pub enum MermaidValidationError {
    EmptyCode,
    RenderError {
        message: String,
        error_type: MermaidErrorType,
        original_code: String,
    },
}

#[derive(Debug, Clone)]
pub enum MermaidErrorType {
    SyntaxError,
    NodeError,
    EdgeError,
    GraphStructureError,
    StyleError,
    Unknown,
}

impl std::fmt::Display for MermaidValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MermaidValidationError::EmptyCode => {
                write!(f, "Mermaid代码为空")
            }
            MermaidValidationError::RenderError { message, error_type, .. } => {
                write!(f, "Mermaid渲染错误 ({:?}): {}", error_type, message)
            }
        }
    }
}

impl std::error::Error for MermaidValidationError {}
