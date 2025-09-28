# 项目分析总结报告（完整版）

生成时间: 2025-09-28 08:16:49 UTC

## 执行耗时统计

- **总执行时间**: 363.19 秒
- **预处理阶段**: 0.11 秒 (0.0%)
- **研究阶段**: 65.77 秒 (18.1%)
- **文档生成阶段**: 297.30 秒 (81.9%)
- **输出阶段**: 0.00 秒 (0.0%)
- **Summary生成时间**: 0.001 秒

## 缓存性能统计与节约效果

### 性能指标
- **缓存命中率**: 58.3%
- **总操作次数**: 24
- **缓存命中**: 14 次
- **缓存未命中**: 10 次
- **缓存写入**: 11 次

### 节约效果
- **节省推理时间**: 62.1 秒
- **节省Token数量**: 16746 输入 + 7614 输出 = 24360 总计
- **估算节省成本**: $0.0130
- **性能提升**: 58.3%
- **效率提升比**: 0.2x（节省时间 / 实际执行时间）

## 核心调研数据汇总

根据Prompt模板数据整合规则，以下为四类调研材料的完整内容：

### 系统上下文调研报告
提供项目的核心目标、用户角色和系统边界信息。

```json
{
  "business_value": "自动化修复技术文档中的 Mermaid 图表错误，减少人工审查成本，提升文档一致性与可读性，适用于大规模文档库的 CI/CD 流程集成。",
  "confidence_score": 0.95,
  "external_systems": [
    {
      "description": "提供 AI 修复能力的远程大语言模型服务，接收 Mermaid 代码片段并返回修复后的版本。",
      "interaction_type": "HTTP API 调用",
      "name": "LLM API（如 Mistral）"
    },
    {
      "description": "存储 Markdown 文件和配置文件（config.toml）的本地磁盘系统。",
      "interaction_type": "文件读写",
      "name": "文件系统"
    },
    {
      "description": "用于注入敏感配置（如 API 密钥）的运行时环境。",
      "interaction_type": "环境变量读取",
      "name": "环境变量"
    }
  ],
  "project_description": "一个命令行工具，用于自动扫描、验证和修复 Markdown 文件中的 Mermaid 图表代码块，通过 AI 能力智能修复语法错误，提升技术文档质量。",
  "project_name": "mermaid-fixer",
  "project_type": "CLITool",
  "system_boundary": {
    "excluded_components": [
      "Mermaid 渲染引擎（仅通过 mermaid-rs 库间接使用）",
      "Web UI 或图形界面",
      "数据库或持久化存储",
      "用户认证系统",
      "网络服务端点",
      "实时协作功能"
    ],
    "included_components": [
      "CLI 参数解析器",
      "配置加载器",
      "Markdown 文件扫描器",
      "Mermaid 语法验证器",
      "AI 修复代理",
      "统计输出工具"
    ],
    "scope": "一个本地命令行工具，专注于 Markdown 文档中 Mermaid 图表的扫描、验证与智能修复。"
  },
  "target_users": [
    {
      "description": "负责编写和维护技术文档的工程师，经常使用 Mermaid 绘制架构图、流程图等。",
      "name": "技术文档工程师",
      "needs": [
        "快速发现并修复 Mermaid 图表语法错误",
        "批量处理多个 Markdown 文件",
        "集成到自动化构建流程中"
      ]
    },
    {
      "description": "在项目中编写技术文档的开发人员，希望减少手动调试图表的时间。",
      "name": "开发者",
      "needs": [
        "在本地或 CI 环境中自动检查文档有效性",
        "获得清晰的错误报告和修复建议",
        "支持自定义 LLM 模型和 API 配置"
      ]
    }
  ]
}
```

### 领域模块调研报告
提供高层次的领域划分、模块关系和核心业务流程信息。

```json
{
  "architecture_summary": "mermaid-fixer 是一个轻量级命令行工具，采用模块化架构设计，核心思想是‘配置驱动 + 职责分离’。系统以 main 为启动中枢，通过 CLI 解析用户输入，加载配置，协调多个独立功能模块完成文档修复任务。各模块间通过清晰的接口解耦，依赖关系单向、明确，形成‘入口-配置-处理-工具’的层次化结构。技术选型上使用 Rust 的标准库与 serde、clap、mermaid-rs 等成熟生态库，确保高性能与可维护性，适合集成至 CI/CD 流程。",
  "business_flows": [
    {
      "description": "用户通过命令行指定目录，系统自动扫描所有 Markdown 文件，提取其中的 Mermaid 图表，验证语法有效性，并对无效图表调用 AI 模型进行智能修复，最终输出统计报告。该流程是系统的核心价值路径，直接实现自动化文档质量提升。",
      "entry_point": "命令行执行 `mermaid-fixer --path <dir>`",
      "importance": 10.0,
      "involved_domains_count": 5,
      "name": "Mermaid 文档批量修复流程",
      "steps": [
        {
          "code_entry_point": null,
          "domain_module": "配置管理域",
          "operation": "解析命令行参数并加载 config.toml 配置文件，合并环境变量覆盖，生成最终配置对象",
          "step": 1,
          "sub_module": null
        },
        {
          "code_entry_point": null,
          "domain_module": "文件扫描域",
          "operation": "递归扫描指定目录，过滤非 Markdown 文件，收集所有 .md 和 .markdown 文件路径",
          "step": 2,
          "sub_module": null
        },
        {
          "code_entry_point": null,
          "domain_module": "语法验证域",
          "operation": "逐个读取 Markdown 文件内容，提取所有 ```mermaid 代码块，调用 MermaidValidator 进行语法验证并分类错误类型",
          "step": 3,
          "sub_module": null
        },
        {
          "code_entry_point": null,
          "domain_module": "AI修复域",
          "operation": "在非干运行模式下，对验证失败的 Mermaid 代码块，调用 AiFixer 向远程 LLM API 发送修复请求，解析响应并获取修复后代码",
          "step": 4,
          "sub_module": null
        },
        {
          "code_entry_point": null,
          "domain_module": "处理协调域",
          "operation": "将修复后的代码写回原文件（若启用），并汇总处理统计信息，通过 utils 模块格式化输出结果",
          "step": 5,
          "sub_module": null
        }
      ]
    },
    {
      "description": "当配置文件 config.toml 不存在时，系统自动创建默认配置，写入磁盘，确保应用可无状态启动。该流程保障了工具的开箱即用性，降低用户使用门槛。",
      "entry_point": "首次运行时检测到 config.toml 不存在",
      "importance": 8.0,
      "involved_domains_count": 2,
      "name": "配置初始化与自动生成流程",
      "steps": [
        {
          "code_entry_point": null,
          "domain_module": "配置管理域",
          "operation": "检测配置文件是否存在，若不存在则加载内置默认配置模板",
          "step": 1,
          "sub_module": null
        },
        {
          "code_entry_point": null,
          "domain_module": "配置管理域",
          "operation": "将默认配置序列化为 TOML 格式并写入 src 目录下的 config.toml 文件",
          "step": 2,
          "sub_module": null
        }
      ]
    }
  ],
  "confidence_score": 0.95,
  "domain_modules": [
    {
      "code_paths": [
        "src/config.rs"
      ],
      "complexity": 7.0,
      "description": "负责应用程序所有配置的加载、合并与管理，包括命令行参数、配置文件、环境变量的优先级处理，是系统行为的决策中枢。提供结构化配置模型，确保各模块获取一致、有效的配置数据。",
      "domain_type": "核心业务域",
      "importance": 9.0,
      "name": "配置管理域",
      "sub_modules": []
    },
    {
      "code_paths": [
        "src/markdown_scanner.rs"
      ],
      "complexity": 4.0,
      "description": "负责递归遍历指定目录，识别并收集所有 Markdown 文件路径，是文档处理的前置入口。其设计为无状态工具，仅依赖标准库，确保高效、稳定、可移植。",
      "domain_type": "工具支撑域",
      "importance": 6.0,
      "name": "文件扫描域",
      "sub_modules": []
    },
    {
      "code_paths": [
        "src/mermaid_validator.rs"
      ],
      "complexity": 8.0,
      "description": "专门负责 Mermaid 图表语法的验证与错误分类，封装底层 mermaid-rs 引擎，提供业务友好的错误枚举，是保证修复准确性的关键质量关卡。",
      "domain_type": "核心业务域",
      "importance": 9.0,
      "name": "语法验证域",
      "sub_modules": []
    },
    {
      "code_paths": [
        "src/ai_fixer.rs"
      ],
      "complexity": 8.0,
      "description": "作为智能代理，负责调用远程 LLM API 实现 Mermaid 代码的自动修复。具备请求构建、响应解析、错误容错与格式兼容能力，是系统智能化的核心体现。",
      "domain_type": "核心业务域",
      "importance": 9.0,
      "name": "AI修复域",
      "sub_modules": []
    },
    {
      "code_paths": [
        "src/processor.rs"
      ],
      "complexity": 6.0,
      "description": "协调整个修复流程，整合扫描、验证、修复等模块的输出，控制执行逻辑（如是否写回文件），并负责最终统计信息的输出。是业务流程的‘指挥中心’，不包含具体业务逻辑。",
      "domain_type": "核心业务域",
      "importance": 8.0,
      "name": "处理协调域",
      "sub_modules": []
    },
    {
      "code_paths": [
        "src/cli.rs"
      ],
      "complexity": 5.0,
      "description": "作为用户交互的唯一入口，负责解析命令行参数并将其映射为内部配置对象。其核心价值是提供灵活、可扩展的用户接口，支持配置优先级管理。",
      "domain_type": "工具支撑域",
      "importance": 7.0,
      "name": "CLI入口域",
      "sub_modules": []
    },
    {
      "code_paths": [
        "src/utils.rs"
      ],
      "complexity": 3.0,
      "description": "提供通用工具函数，如从 Markdown 内容中提取 Mermaid 代码块、格式化输出统计信息。这些函数为上层模块提供纯函数式辅助能力，不参与核心流程控制。",
      "domain_type": "工具支撑域",
      "importance": 5.0,
      "name": "工具支持域",
      "sub_modules": []
    }
  ],
  "domain_relations": [
    {
      "description": "CLI 模块解析用户参数后，将结果转换为 Config 配置对象，作为后续所有模块的输入源。",
      "from_domain": "CLI入口域",
      "relation_type": "数据依赖",
      "strength": 9.0,
      "to_domain": "配置管理域"
    },
    {
      "description": "文件扫描模块依赖配置中的目录路径、排除规则（如 .git）等参数，决定扫描范围。",
      "from_domain": "配置管理域",
      "relation_type": "配置依赖",
      "strength": 7.0,
      "to_domain": "文件扫描域"
    },
    {
      "description": "语法验证模块依赖配置中的超时时间等参数，控制验证行为。",
      "from_domain": "配置管理域",
      "relation_type": "配置依赖",
      "strength": 8.0,
      "to_domain": "语法验证域"
    },
    {
      "description": "AI修复模块必须依赖配置中的 LLM API 密钥、模型名称和基础 URL 才能发起请求。",
      "from_domain": "配置管理域",
      "relation_type": "配置依赖",
      "strength": 9.0,
      "to_domain": "AI修复域"
    },
    {
      "description": "处理协调模块依赖配置决定是否启用修复、是否写回文件等关键执行策略。",
      "from_domain": "配置管理域",
      "relation_type": "配置依赖",
      "strength": 8.0,
      "to_domain": "处理协调域"
    },
    {
      "description": "CLI 解析后调用处理协调模块的主处理函数，启动整个修复流程。",
      "from_domain": "CLI入口域",
      "relation_type": "服务调用",
      "strength": 8.0,
      "to_domain": "处理协调域"
    },
    {
      "description": "处理协调模块主动调用文件扫描模块，获取待处理的 Markdown 文件列表。",
      "from_domain": "处理协调域",
      "relation_type": "服务调用",
      "strength": 9.0,
      "to_domain": "文件扫描域"
    },
    {
      "description": "处理协调模块对每个扫描到的文件调用语法验证模块，判断图表是否有效。",
      "from_domain": "处理协调域",
      "relation_type": "服务调用",
      "strength": 9.0,
      "to_domain": "语法验证域"
    },
    {
      "description": "当配置允许修复且验证失败时，处理协调模块调用 AI 修复模块进行自动修复。",
      "from_domain": "处理协调域",
      "relation_type": "服务调用",
      "strength": 8.0,
      "to_domain": "AI修复域"
    },
    {
      "description": "处理协调模块调用工具支持模块中的 print_statistics 函数输出最终结果。",
      "from_domain": "处理协调域",
      "relation_type": "服务调用",
      "strength": 7.0,
      "to_domain": "工具支持域"
    },
    {
      "description": "文件扫描模块在提取文件内容时，调用 extract_mermaid_blocks 工具函数来定位代码块。",
      "from_domain": "文件扫描域",
      "relation_type": "服务调用",
      "strength": 6.0,
      "to_domain": "工具支持域"
    },
    {
      "description": "AI修复模块在构建请求时，需读取配置中的 API 密钥、模型等信息。",
      "from_domain": "AI修复域",
      "relation_type": "数据依赖",
      "strength": 9.0,
      "to_domain": "配置管理域"
    },
    {
      "description": "语法验证模块在初始化时读取配置中的超时参数。",
      "from_domain": "语法验证域",
      "relation_type": "数据依赖",
      "strength": 7.0,
      "to_domain": "配置管理域"
    }
  ]
}
```

### 工作流调研报告
包含对代码库的静态分析结果和业务流程分析。

```json
{
  "main_workflow": {
    "description": "用户通过命令行指定目录，系统自动扫描所有 Markdown 文件，提取其中的 Mermaid 图表，验证语法有效性，并对无效图表调用 AI 模型进行智能修复，最终输出统计报告。该流程是系统的核心价值路径，直接实现自动化文档质量提升。",
    "flowchart_mermaid": "graph TD\n    A[用户执行命令: mermaid-fixer --path <dir>] --> B[CLI入口域解析参数]\n    B --> C[配置管理域加载并合并配置（CLI/文件/环境变量）]\n    C --> D[处理协调域启动主流程]\n    D --> E[文件扫描域递归扫描目录，收集所有.md/.markdown文件]\n    E --> F[对每个文件：提取Mermaid代码块]\n    F --> G[语法验证域验证每个代码块语法有效性]\n    G --> H{是否有效？}\n    H -- 是 --> I[跳过，记录成功]\n    H -- 否 --> J[AI修复域调用LLM API请求修复]\n    J --> K[处理协调域判断是否启用写回]\n    K -- 是 --> L[将修复后代码写回原文件]\n    K -- 否 --> M[仅记录修复建议]\n    I --> N[汇总所有处理结果]\n    M --> N\n    L --> N\n    N --> O[工具支持域输出统计报告]\n    O --> P[流程结束]",
    "name": "Mermaid 文档批量修复流程"
  },
  "other_important_workflows": [
    {
      "description": "当配置文件 config.toml 不存在时，系统自动创建默认配置，写入磁盘，确保应用可无状态启动。该流程保障了工具的开箱即用性，降低用户使用门槛。",
      "flowchart_mermaid": "graph TD\n    A[首次运行系统] --> B[配置管理域检测config.toml是否存在]\n    B -- 存在 --> C[加载配置，跳过本流程]\n    B -- 不存在 --> D[加载内置默认配置模板]\n    D --> E[序列化为TOML格式]\n    E --> F[写入当前目录下的config.toml文件]\n    F --> G[流程结束，继续主流程]",
      "name": "配置初始化与自动生成流程"
    }
  ]
}
```

### 代码洞察数据
来自预处理阶段的代码分析结果，包含函数、类和模块的定义。

```json
[
  {
    "code_dossier": {
      "code_purpose": "entry",
      "description": null,
      "file_path": "src/main.rs",
      "functions": [
        "main"
      ],
      "importance_score": 1.0,
      "interfaces": [],
      "name": "main.rs",
      "source_summary": "use clap::Parser;\nuse std::process;\n\nmod cli;\nmod config;\nmod markdown_scanner;\nmod mermaid_validator;\nmod ai_fixer;\nmod processor;\nmod utils;\n\nuse cli::Args;\nuse processor::MermaidProcessor;\nuse utils::print_statistics;\n\n#[tokio::main]\nasync fn main() {\n    let args = Args::parse();\n    \n    // 提取需要的值\n    let directory = args.directory.clone();\n    let dry_run = args.dry_run;\n    let verbose = args.verbose;\n    \n    // 解析配置\n    let config = match args.to_config() {\n        Ok(config) => config,\n        Err(e) => {\n            eprintln!(\"❌ 配置错误: {}\", e);\n            process::exit(1);\n        }\n    };\n\n    // 创建处理器\n    let processor = match MermaidProcessor::new(&config, dry_run, verbose).await {\n        Ok(processor) => processor,\n        Err(e) => {\n            eprintln!(\"❌ 初始化失败: {}\", e);\n            process::exit(1);\n        }\n    };\n\n    // 处理目录\n    let result = match processor.process_directory(directory, dry_run).await {\n        Ok(result) => result,\n        Err(e) => {\n            eprintln!(\"❌ 处理失败: {}\", e);\n            process::exit(1);\n        }\n    };\n\n    // 输出统计信息\n    print_statistics(&result, dry_run);\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.9,
      "coupling_factor": 0.7,
      "cyclomatic_complexity": 4.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 54,
      "number_of_classes": 0,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "clap",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "tokio",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "std::process",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "cli",
        "path": "./src/cli.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "config",
        "path": "./src/config.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "mermaid_validator",
        "path": "./src/mermaid_validator.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "ai_fixer",
        "path": "./src/ai_fixer.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "processor",
        "path": "./src/processor.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "utils",
        "path": "./src/utils.rs",
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "markdown_scanner",
        "path": "./src/markdown_scanner.rs",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "Args",
        "path": "./src/cli.rs",
        "version": null
      },
      {
        "dependency_type": "struct",
        "is_external": false,
        "line_number": null,
        "name": "MermaidProcessor",
        "path": "./src/processor.rs",
        "version": null
      }
    ],
    "detailed_description": "main.rs 是项目的执行入口，负责协调整个应用程序的启动流程。它通过解析命令行参数（CLI）获取用户配置，加载配置文件（config.toml），初始化 MermaidProcessor 处理器，并调用其异步方法处理指定目录下的 Markdown 文件。整个流程采用错误驱动的结构，所有关键操作（配置加载、处理器初始化、目录处理）均使用 match 表达式进行错误处理，失败时打印错误信息并退出进程。最终输出处理统计信息。该组件不包含业务逻辑本身，而是作为‘胶水代码’连接各模块，是整个应用的启动中枢。",
    "interfaces": [],
    "responsibilities": [
      "解析命令行参数并转换为配置对象",
      "初始化 MermaidProcessor 处理器实例",
      "协调异步处理流程，调用 processor.process_directory() 执行核心任务",
      "统一处理关键操作中的错误并优雅退出",
      "输出最终的处理统计信息"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "config",
      "description": null,
      "file_path": "src/config.rs",
      "functions": [
        "load"
      ],
      "importance_score": 0.9,
      "interfaces": [
        "Config",
        "LlmConfig",
        "MermaidConfig"
      ],
      "name": "config.rs",
      "source_summary": "use serde::{Deserialize, Serialize};\nuse std::fs;\nuse std::path::Path;\n\n#[derive(Debug, Deserialize, Serialize)]\npub struct Config {\n    pub llm: LlmConfig,\n    pub mermaid: MermaidConfig,\n}\n\n#[derive(Debug, Deserialize, Serialize)]\npub struct LlmConfig {\n    pub provider: String,\n    pub model: String,\n    pub api_key: Option<String>,\n    pub base_url: Option<String>,\n    pub max_tokens: Option<u32>,\n    pub temperature: Option<f32>,\n}\n\n#[derive(Debug, Deserialize, Serialize)]\npub struct MermaidConfig {\n    pub timeout_seconds: Option<u64>,\n    pub max_retries: Option<u32>,\n}\n\nimpl Default for Config {\n    fn default() -> Self {\n        Self {\n            llm: LlmConfig {\n                provider: \"mistral\".to_string(),\n                model: \"mistral-small-latest\".to_string(),\n                api_key: None,\n                base_url: None,\n                max_tokens: Some(65536),\n                temperature: Some(0.1),\n            },\n            mermaid: MermaidConfig {\n                timeout_seconds: Some(120),\n                max_retries: Some(3),\n            },\n        }\n    }\n}\n\nimpl Config {\n    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {\n        let path = path.as_ref();\n        \n        if !path.exists() {\n            // 如果配置文件不存在，创建默认配置文件\n            let default_config = Self::default();\n            let toml_content = toml::to_string_pretty(&default_config)?;\n            fs::write(path, toml_content)?;\n            println!(\"📝 已创建默认配置文件: {}\", path.display());\n            return Ok(default_config);\n        }\n\n        let content = fs::read_to_string(path)?;\n        let mut config: Config = toml::from_str(&content)?;\n        \n        // 从环境变量中读取API密钥（如果配置文件中没有设置）\n        if config.llm.api_key.is_none() {\n            config.llm.api_key = std::env::var(\"LITHO_LLM_API_KEY\").ok();\n        }\n\n        Ok(config)\n    }\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.3,
      "cyclomatic_complexity": 4.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 69,
      "number_of_classes": 3,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "serde",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "library",
        "is_external": true,
        "line_number": null,
        "name": "toml",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": null,
        "name": "std::fs",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件负责应用程序的配置管理，提供结构化配置模型（Config、LlmConfig、MermaidConfig）并实现从TOML文件加载配置的逻辑。支持默认配置自动生成、环境变量覆盖（LITHO_LLM_API_KEY）以及错误处理。当配置文件不存在时，自动创建默认配置并写入磁盘，确保应用启动时始终有有效配置。使用serde进行序列化/反序列化，提升配置读写效率与可维护性。",
    "interfaces": [
      {
        "description": null,
        "interface_type": "struct",
        "name": "Config",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "llm",
            "param_type": "LlmConfig"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "mermaid",
            "param_type": "MermaidConfig"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "LlmConfig",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "provider",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "model",
            "param_type": "String"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "api_key",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "base_url",
            "param_type": "Option<String>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "max_tokens",
            "param_type": "Option<u32>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "temperature",
            "param_type": "Option<f32>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": null,
        "interface_type": "struct",
        "name": "MermaidConfig",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "timeout_seconds",
            "param_type": "Option<u64>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "max_retries",
            "param_type": "Option<u32>"
          }
        ],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "定义应用配置数据结构（Config、LlmConfig、MermaidConfig）",
      "从TOML文件加载配置并处理文件不存在场景",
      "自动生成默认配置并写入磁盘",
      "从环境变量LITHO_LLM_API_KEY覆盖配置中的API密钥",
      "提供类型安全的配置访问接口"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": null,
      "file_path": "src/processor.rs",
      "functions": [
        "MermaidProcessor::new",
        "MermaidProcessor::process_directory",
        "MermaidProcessor::process_file",
        "FileProcessResult::default"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "MermaidProcessor",
        "ProcessResult",
        "FileProcessResult"
      ],
      "name": "processor.rs",
      "source_summary": "use std::path::PathBuf;\n\nuse crate::config::Config;\nuse crate::markdown_scanner::MarkdownScanner;\nuse crate::mermaid_validator::MermaidValidator;\nuse crate::ai_fixer::AiFixer;\nuse crate::utils::extract_mermaid_blocks;\n\npub struct MermaidProcessor {\n    scanner: MarkdownScanner,\n    validator: MermaidValidator,\n    ai_fixer: Option<AiFixer>,\n    verbose: bool,\n}\n\npub struct ProcessResult {\n    pub total_files: usize,\n    pub total_mermaid_blocks: usize,\n    pub invalid_blocks: usize,\n    pub fixed_blocks: usize,\n}\n\nimpl MermaidProcessor {\n    pub async fn new(config: &Config, dry_run: bool, verbose: bool) -> Result<Self, Box<dyn std::error::Error>> {\n        let scanner = MarkdownScanner::new();\n        \n        let validator = MermaidValidator::with_config(config.mermaid.timeout_seconds)?;\n        \n        let ai_fixer = if dry_run {\n            None\n        } else {\n            Some(AiFixer::new(config).await?)\n        };\n\n        Ok(Self {\n            scanner,\n            validator,\n            ai_fixer,\n            verbose,\n        })\n    }\n\n    pub async fn process_directory(&self, directory: PathBuf, dry_run: bool) -> Result<ProcessResult, Box<dyn std::error::Error>> {\n        if self.verbose {\n            println!(\"🚀 开始扫描目录: {}\", directory.display());\n        }\n        \n        // 扫描markdown文件\n        let markdown_files = self.scanner.scan_directory(&directory)?;\n        \n        if self.verbose {\n            println!(\"📄 找到 {} 个markdown文件\", markdown_files.len());\n        }\n\n        let mut total_mermaid_blocks = 0;\n        let mut invalid_blocks = 0;\n        let mut fixed_blocks = 0;\n\n        // 处理每个markdown文件\n        for file_path in &markdown_files {\n            if self.verbose {\n                println!(\"\\n📝 处理文件: {}\", file_path.display());\n            }\n            \n            let result = self.process_file(file_path, dry_run).await?;\n            \n            total_mermaid_blocks += result.total_blocks;\n            invalid_blocks += result.invalid_blocks;\n            fixed_blocks += result.fixed_blocks;\n        }\n\n        Ok(ProcessResult {\n            total_files: markdown_files.len(),\n            total_mermaid_blocks,\n            invalid_blocks,\n            fixed_blocks,\n        })\n    }\n\n    async fn process_file(&self, file_path: &PathBuf, dry_run: bool) -> Result<FileProcessResult, Box<dyn std::error::Error>> {\n        // 读取文件内容\n        let content = std::fs::read_to_string(file_path)?;\n\n        // 提取mermaid代码块\n        let mermaid_blocks = extract_mermaid_blocks(&content);\n        \n        if mermaid_blocks.is_empty() {\n            if self.verbose {\n                println!(\"   ℹ️  未找到mermaid代码块\");\n            }\n            return Ok(FileProcessResult::default());\n        }\n\n        if self.verbose {\n            println!(\"   🔍 找到 {} 个mermaid代码块\", mermaid_blocks.len());\n        }\n\n        let mut invalid_blocks = 0;\n        let mut fixed_blocks = 0;\n        let mut file_modified = false;\n        let mut new_content = content.clone();\n\n        // 验证每个mermaid代码块\n        for (index, (_start_pos, _end_pos, mermaid_code)) in mermaid_blocks.iter().enumerate() {\n            if self.verbose {\n                println!(\"      📊 验证代码块 {}/{}\", index + 1, mermaid_blocks.len());\n            }\n            \n            let validator = MermaidValidator::with_config(None)?;\n            \n            match validator.validate(mermaid_code) {\n                Ok(_) => {\n                    if self.verbose {\n                        println!(\"         ✅ 代码块有效\");\n                    }\n                }\n                Err(e) => {\n                    if self.verbose {\n                        println!(\"         ❌ 代码块无效: {}\", e);\n                    }\n                    invalid_blocks += 1;\n\n                    if !dry_run {\n                        if let Some(ai_fixer) = &self.ai_fixer {\n                            match ai_fixer.fix_mermaid(mermaid_code).await {\n                                Ok(fixed_code) => {\n                                    // 验证修复后的代码\n                                    match validator.validate(&fixed_code) {\n                                        Ok(_) => {\n                                            if self.verbose {\n                                                println!(\"         🔧 修复成功\");\n                                            }\n                                            // 替换原始代码\n                                            new_content = new_content.replace(mermaid_code, &fixed_code);\n                                            file_modified = true;\n                                            fixed_blocks += 1;\n                                        }\n                                        Err(validation_error) => {\n                                            if self.verbose {\n                                                println!(\"         ⚠️  修复后的代码仍然无效: {}\", validation_error);\n                                            }\n                                        }\n                                    }\n                                }\n                                Err(fix_error) => {\n                                    if self.verbose {\n                                        println!(\"         ⚠️  AI修复失败: {}\", fix_error);\n                                    }\n                                }\n                            }\n                        }\n                    }\n                }\n            }\n        }\n\n        // 如果文件被修改，写回文件\n        if file_modified {\n            std::fs::write(file_path, new_content)?;\n            if self.verbose {\n                println!(\"   💾 文件已更新\");\n            }\n        }\n\n        Ok(FileProcessResult {\n            total_blocks: mermaid_blocks.len(),\n            invalid_blocks,\n            fixed_blocks,\n        })\n    }\n}\n\n#[derive(Default)]\nstruct FileProcessResult {\n    total_blocks: usize,\n    invalid_blocks: usize,\n    fixed_blocks: usize,\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.7,
      "cyclomatic_complexity": 23.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 178,
      "number_of_classes": 3,
      "number_of_functions": 4
    },
    "dependencies": [
      {
        "dependency_type": "direct",
        "is_external": false,
        "line_number": 5,
        "name": "Config",
        "path": "src/config.rs",
        "version": null
      },
      {
        "dependency_type": "direct",
        "is_external": false,
        "line_number": 6,
        "name": "MarkdownScanner",
        "path": "src/markdown_scanner.rs",
        "version": null
      },
      {
        "dependency_type": "direct",
        "is_external": false,
        "line_number": 7,
        "name": "MermaidValidator",
        "path": "src/mermaid_validator.rs",
        "version": null
      },
      {
        "dependency_type": "direct",
        "is_external": false,
        "line_number": 8,
        "name": "AiFixer",
        "path": "src/ai_fixer.rs",
        "version": null
      },
      {
        "dependency_type": "direct",
        "is_external": false,
        "line_number": 9,
        "name": "extract_mermaid_blocks",
        "path": "src/utils/extract_mermaid_blocks.rs",
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": true,
        "line_number": 1,
        "name": "std::path::PathBuf",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "MermaidProcessor 是一个专门用于扫描、验证和自动修复 Markdown 文件中 Mermaid 图表代码块的工具组件。它通过集成 MarkdownScanner 提取所有 Mermaid 代码块，使用 MermaidValidator 验证其语法正确性，并在非干运行模式下调用 AiFixer 尝试自动修复无效的代码块。该组件支持详细日志输出，可选择性地修改原文件，适用于自动化文档质量检查与修复场景。核心流程包括：扫描目录 → 提取代码块 → 验证语法 → 条件修复 → 写回文件。其设计支持配置驱动（如超时、AI修复开关）和异步操作，适用于大型文档库的批量处理。",
    "interfaces": [
      {
        "description": "主处理器结构体，封装了扫描、验证和修复逻辑，通过依赖注入方式解耦各子模块",
        "interface_type": "struct",
        "name": "MermaidProcessor",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "scanner",
            "param_type": "MarkdownScanner"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "validator",
            "param_type": "MermaidValidator"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "ai_fixer",
            "param_type": "Option<AiFixer>"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "verbose",
            "param_type": "bool"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "表示整个目录处理结果的聚合统计结构，用于返回批量处理的概要信息",
        "interface_type": "struct",
        "name": "ProcessResult",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "total_files",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "total_mermaid_blocks",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "invalid_blocks",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "fixed_blocks",
            "param_type": "usize"
          }
        ],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "表示单个文件处理结果的统计结构，用于 process_file 方法的返回值",
        "interface_type": "struct",
        "name": "FileProcessResult",
        "parameters": [
          {
            "description": null,
            "is_optional": false,
            "name": "total_blocks",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "invalid_blocks",
            "param_type": "usize"
          },
          {
            "description": null,
            "is_optional": false,
            "name": "fixed_blocks",
            "param_type": "usize"
          }
        ],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "扫描指定目录下的所有 Markdown 文件并提取 Mermaid 代码块",
      "验证每个 Mermaid 代码块的语法正确性",
      "在非干运行模式下，调用 AI 服务自动修复无效的 Mermaid 代码块",
      "记录处理统计信息（总块数、无效块数、修复块数）并选择性地写回修改后的文件",
      "提供详细日志输出以支持调试和用户反馈"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src/markdown_scanner.rs",
      "functions": [
        "new",
        "scan_directory",
        "scan_recursive",
        "is_markdown_file",
        "should_skip_directory"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "markdown_scanner.rs",
      "source_summary": "use std::fs;\nuse std::path::{Path, PathBuf};\n\npub struct MarkdownScanner {\n    // 可以添加配置选项，比如忽略的目录等\n}\n\nimpl MarkdownScanner {\n    pub fn new() -> Self {\n        Self {}\n    }\n\n    /// 扫描指定目录下的所有markdown文件\n    pub fn scan_directory<P: AsRef<Path>>(&self, dir: P) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {\n        let mut markdown_files = Vec::new();\n        self.scan_recursive(dir.as_ref(), &mut markdown_files)?;\n        Ok(markdown_files)\n    }\n\n    /// 递归扫描目录\n    fn scan_recursive(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {\n        if !dir.is_dir() {\n            return Err(format!(\"路径不是目录: {}\", dir.display()).into());\n        }\n\n        let entries = fs::read_dir(dir)?;\n\n        for entry in entries {\n            let entry = entry?;\n            let path = entry.path();\n\n            if path.is_dir() {\n                // 跳过一些常见的不需要扫描的目录\n                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {\n                    if self.should_skip_directory(dir_name) {\n                        continue;\n                    }\n                }\n                \n                // 递归扫描子目录\n                self.scan_recursive(&path, files)?;\n            } else if path.is_file() {\n                // 检查是否是markdown文件\n                if self.is_markdown_file(&path) {\n                    files.push(path);\n                }\n            }\n        }\n\n        Ok(())\n    }\n\n    /// 判断是否是markdown文件\n    fn is_markdown_file(&self, path: &Path) -> bool {\n        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {\n            matches!(extension.to_lowercase().as_str(), \"md\" | \"markdown\")\n        } else {\n            false\n        }\n    }\n\n    /// 判断是否应该跳过某个目录\n    fn should_skip_directory(&self, dir_name: &str) -> bool {\n        matches!(\n            dir_name,\n            \".git\" | \".svn\" | \".hg\" | \n            \"node_modules\" | \"target\" | \"build\" | \"dist\" |\n            \".vscode\" | \".idea\" | \".vs\" |\n            \"__pycache__\" | \".pytest_cache\" |\n            \"vendor\" | \"deps\"\n        )\n    }\n}\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.2,
      "cyclomatic_complexity": 9.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 73,
      "number_of_classes": 1,
      "number_of_functions": 5
    },
    "dependencies": [
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": null,
        "name": "std::fs",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "standard_library",
        "is_external": false,
        "line_number": null,
        "name": "std::path",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "MarkdownScanner 是一个用于递归扫描指定目录并收集所有 Markdown 文件（.md 和 .markdown）的工具类。它通过遍历目录树，跳过常见的版本控制和构建目录（如 .git、node_modules、target 等），并基于文件扩展名判断是否为 Markdown 文件。该组件不依赖外部库，仅使用标准库的 fs 和 path 模块，提供无状态的纯功能扫描能力，适用于文档索引、静态网站生成或内容管理系统等场景。",
    "interfaces": [],
    "responsibilities": [
      "递归遍历文件系统目录",
      "识别并过滤 Markdown 文件（.md/.markdown）",
      "跳过预定义的系统/构建目录（如 .git、node_modules 等）",
      "封装文件系统操作异常处理",
      "提供简洁的 API 接口供外部调用"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "specificfeature",
      "description": null,
      "file_path": "src/mermaid_validator.rs",
      "functions": [
        "with_config",
        "validate",
        "preprocess_code",
        "classify_error"
      ],
      "importance_score": 0.8,
      "interfaces": [
        "MermaidValidator",
        "MermaidValidationError",
        "MermaidErrorType"
      ],
      "name": "mermaid_validator.rs",
      "source_summary": "use mermaid_rs::Mermaid;\nuse std::time::Duration;\n\npub struct MermaidValidator {\n    mermaid: Mermaid,\n    timeout: Duration,\n}\n\nimpl MermaidValidator {\n\n    pub fn with_config(timeout_seconds: Option<u64>) -> Result<Self, Box<dyn std::error::Error>> {\n        let mermaid = Mermaid::new()\n            .map_err(|e| format!(\"初始化Mermaid实例失败: {}\", e))?;\n        \n        let timeout = Duration::from_secs(timeout_seconds.unwrap_or(30));\n        \n        Ok(Self {\n            mermaid,\n            timeout,\n        })\n    }\n\n    /// 验证mermaid代码是否有效\n    pub fn validate(&self, mermaid_code: &str) -> Result<(), MermaidValidationError> {\n        if mermaid_code.trim().is_empty() {\n            return Err(MermaidValidationError::EmptyCode);\n        }\n\n        // 预处理代码：移除可能的前后空白和注释\n        let cleaned_code = self.preprocess_code(mermaid_code);\n\n        // 使用mermaid-rs进行验证\n        match self.mermaid.render(&cleaned_code) {\n            Ok(_) => Ok(()),\n            Err(e) => {\n                // 分析错误类型\n                let error_message = e.to_string();\n                let error_type = self.classify_error(&error_message);\n                Err(MermaidValidationError::RenderError {\n                    message: error_message,\n                    error_type,\n                    original_code: mermaid_code.to_string(),\n                })\n            }\n        }\n    }\n\n    /// 预处理mermaid代码\n    fn preprocess_code(&self, code: &str) -> String {\n        code.lines()\n            .map(|line| line.trim())\n            .filter(|line| !line.is_empty() && !line.starts_with(\"%%\"))\n            .collect::<Vec<_>>()\n            .join(\"\\n\")\n    }\n\n    /// 分类错误类型\n    fn classify_error(&self, error_message: &str) -> MermaidErrorType {\n        let error_lower = error_message.to_lowercase();\n        \n        if error_lower.contains(\"syntax\") || error_lower.contains(\"parse\") {\n            MermaidErrorType::SyntaxError\n        } else if error_lower.contains(\"node\") || error_lower.contains(\"vertex\") {\n            MermaidErrorType::NodeError\n        } else if error_lower.contains(\"edge\") || error_lower.contains(\"arrow\") || error_lower.contains(\"link\") {\n            MermaidErrorType::EdgeError\n        } else if error_lower.contains(\"graph\") || error_lower.contains(\"diagram\") {\n            MermaidErrorType::GraphStructureError\n        } else if error_lower.contains(\"style\") || error_lower.contains(\"class\") {\n            MermaidErrorType::StyleError\n        } else {\n            MermaidErrorType::Unknown\n        }\n    }\n}\n\n#[derive(Debug, Clone)]\npub enum MermaidValidationError {\n    EmptyCode,\n    RenderError {\n        message: String,\n        error_type: MermaidErrorType,\n        original_code: String,\n    },\n}\n\n#[derive(Debug, Clone)]\npub enum MermaidErrorType {\n    SyntaxError,\n    NodeError,\n    EdgeError,\n    GraphStructureError,\n    StyleError,\n    Unknown,\n}\n\nimpl std::fmt::Display for MermaidValidationError {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        match self {\n            MermaidValidationError::EmptyCode => {\n                write!(f, \"Mermaid代码为空\")\n            }\n            MermaidValidationError::RenderError { message, error_type, .. } => {\n                write!(f, \"Mermaid渲染错误 ({:?}): {}\", error_type, message)\n            }\n        }\n    }\n}\n\nimpl std::error::Error for MermaidValidationError {}\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.3,
      "cyclomatic_complexity": 11.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 110,
      "number_of_classes": 1,
      "number_of_functions": 4
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "mermaid_rs",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "stdlib",
        "is_external": true,
        "line_number": null,
        "name": "std::time::Duration",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "MermaidValidator 是一个用于验证 Mermaid 图表代码有效性的专用组件。它封装了 mermaid-rs 库的渲染功能，提供预处理（移除空白行和注释）、错误分类和统一错误返回机制。组件通过 with_config 构造函数初始化，支持自定义超时时间（默认30秒）。validate 方法接收 Mermaid 代码字符串，经过预处理后调用底层渲染引擎，根据渲染结果返回成功或结构化错误信息。错误被细分为语法、节点、边、图结构、样式等类型，便于上层进行针对性处理。该组件不直接暴露底层库的原始错误，而是将其转换为业务友好的枚举类型，提升了错误处理的一致性和可维护性。",
    "interfaces": [
      {
        "description": "核心验证器结构体，封装 Mermaid 实例和超时配置",
        "interface_type": "struct",
        "name": "MermaidValidator",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "定义验证失败的两种错误类型：空代码和渲染错误",
        "interface_type": "enum",
        "name": "MermaidValidationError",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      },
      {
        "description": "细粒度分类渲染错误的具体原因，如语法错误、节点错误等",
        "interface_type": "enum",
        "name": "MermaidErrorType",
        "parameters": [],
        "return_type": null,
        "visibility": "public"
      }
    ],
    "responsibilities": [
      "封装 mermaid-rs 渲染逻辑，提供统一验证接口",
      "预处理 Mermaid 代码（去除空行和注释）",
      "智能分类渲染错误类型（语法、节点、边等）",
      "提供结构化错误枚举，增强错误可读性和可处理性",
      "支持配置化超时设置，提升组件灵活性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "agent",
      "description": null,
      "file_path": "src/ai_fixer.rs",
      "functions": [
        "new",
        "fix_mermaid",
        "build_prompt",
        "extract_fixed_code",
        "clean_response",
        "extract_code_block"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "ai_fixer.rs",
      "source_summary": "use serde::{Deserialize, Serialize};\n\nuse crate::config::Config;\n\npub struct AiFixer {\n    api_key: String,\n    model: String,\n    prompt_template: String,\n    base_url: String,\n}\n\n#[derive(Debug, Serialize, Deserialize)]\nstruct OpenAIRequest {\n    model: String,\n    messages: Vec<Message>,\n    max_tokens: Option<u32>,\n    temperature: Option<f32>,\n}\n\n#[derive(Debug, Serialize, Deserialize)]\nstruct Message {\n    role: String,\n    content: String,\n}\n\n#[derive(Debug, Serialize, Deserialize)]\nstruct OpenAIResponse {\n    choices: Vec<Choice>,\n}\n\n#[derive(Debug, Serialize, Deserialize)]\nstruct Choice {\n    message: Message,\n}\n\n#[derive(Debug, Serialize, Deserialize)]\nstruct FixResponse {\n    fixed_code: String,\n    explanation: String,\n    changes: Option<Vec<ChangeDetail>>,\n}\n\n#[derive(Debug, Serialize, Deserialize)]\nstruct ChangeDetail {\n    #[serde(rename = \"type\")]\n    change_type: String,\n    original: String,\n    fixed: String,\n    reason: String,\n}\n\nimpl AiFixer {\n    pub async fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {\n        // 获取API密钥\n        let api_key = config.llm.api_key.clone()\n            .ok_or(\"未找到LLM API密钥，请通过参数或配置文件指定\")?;\n\n        // 读取prompt模板\n        let prompt_template = include_str!(\"prompt.tpl\").to_owned();\n\n        let base_url = config.llm.base_url.clone()\n            .unwrap_or_else(|| \"https://api.mistral.ai/v1\".to_string());\n\n        Ok(Self {\n            api_key,\n            model: config.llm.model.clone(),\n            prompt_template,\n            base_url,\n        })\n    }\n\n    /// 修复mermaid代码\n    pub async fn fix_mermaid(&self, mermaid_code: &str) -> Result<String, Box<dyn std::error::Error>> {\n        // 构建提示词\n        let prompt = self.build_prompt(mermaid_code);\n        \n        // 构建请求\n        let request = OpenAIRequest {\n            model: self.model.clone(),\n            messages: vec![\n                Message {\n                    role: \"user\".to_string(),\n                    content: prompt,\n                }\n            ],\n            max_tokens: Some(65536),\n            temperature: Some(0.1),\n        };\n\n        // 发送HTTP请求\n        let client = reqwest::Client::new();\n        let response = client\n            .post(&format!(\"{}/chat/completions\", self.base_url))\n            .header(\"Authorization\", format!(\"Bearer {}\", self.api_key))\n            .header(\"Content-Type\", \"application/json\")\n            .json(&request)\n            .send()\n            .await?;\n\n        if !response.status().is_success() {\n            let error_text = response.text().await?;\n            return Err(format!(\"LLM API请求失败: {}\", error_text).into());\n        }\n\n        let openai_response: OpenAIResponse = response.json().await?;\n        \n        if openai_response.choices.is_empty() {\n            return Err(\"LLM API返回空响应\".into());\n        }\n\n        let content = &openai_response.choices[0].message.content;\n        \n        // 解析响应，提取修复后的代码\n        let fixed_code = self.extract_fixed_code(content)?;\n        \n        Ok(fixed_code)\n    }\n\n    /// 构建提示词\n    fn build_prompt(&self, mermaid_code: &str) -> String {\n        self.prompt_template.replace(\"{{MERMAID_CODE}}\", mermaid_code)\n    }\n\n    /// 从LLM响应中提取修复后的代码\n    fn extract_fixed_code(&self, response: &str) -> Result<String, Box<dyn std::error::Error>> {\n        // 清理响应内容，移除可能的markdown代码块包装\n        let cleaned_response = self.clean_response(response);\n        \n        // 尝试解析JSON格式的响应\n        if let Ok(fix_response) = serde_json::from_str::<FixResponse>(&cleaned_response) {\n            println!(\"         📋 修复说明: {}\", fix_response.explanation);\n            if let Some(changes) = &fix_response.changes {\n                for (i, change) in changes.iter().enumerate() {\n                    println!(\"         🔧 修改 {}: {} -> {}\", \n                        i + 1, change.change_type, change.reason);\n                }\n            }\n            return Ok(fix_response.fixed_code);\n        }\n\n        // 如果不是JSON格式，尝试提取markdown代码块\n        if let Some(code) = self.extract_code_block(response) {\n            return Ok(code);\n        }\n\n        // 如果都失败了，返回原始响应（去除前后空白）\n        Ok(response.trim().to_string())\n    }\n\n    /// 清理响应内容，移除可能的markdown包装\n    fn clean_response(&self, response: &str) -> String {\n        let response = response.trim();\n        \n        // 如果响应被```json包装，提取其中的JSON内容\n        if response.starts_with(\"```json\") && response.ends_with(\"```\") {\n            let lines: Vec<&str> = response.lines().collect();\n            if lines.len() > 2 {\n                return lines[1..lines.len()-1].join(\"\\n\");\n            }\n        }\n        \n        // 如果响应被```包装，提取其中的内容\n        if response.starts_with(\"```\") && response.ends_with(\"```\") {\n            let lines: Vec<&str> = response.lines().collect();\n            if lines.len() > 2 {\n                return lines[1..lines.len()-1].join(\"\\n\");\n            }\n        }\n        \n        response.to_string()\n    }\n\n    /// 从markdown格式的响应中提取代码块\n    fn extract_code_block(&self, response: &str) -> Option<String> {\n        let lines: Vec<&str> = response.lines().collect();\n        let mut in_code_block = false;\n        let mut code_lines = Vec::new();\n        \n        for line in lines {\n            if line.trim().starts_with(\"```mermaid\") {\n                in_code_block = true;\n                continue;\n            }\n            \n            if line.trim() == \"```\" && in_code_block {\n                break;\n            }\n            \n            if in_code_block {\n                code_lines.push(line);\n            }\n        }\n        \n        if !code_lines.is_empty() {\n            Some(code_lines.join(\"\\n\"))\n        } else {\n            None\n        }\n    }\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.85,
      "coupling_factor": 0.6,
      "cyclomatic_complexity": 16.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 200,
      "number_of_classes": 1,
      "number_of_functions": 6
    },
    "dependencies": [
      {
        "dependency_type": "internal",
        "is_external": false,
        "line_number": null,
        "name": "Config",
        "path": "src/config.rs",
        "version": null
      },
      {
        "dependency_type": "external",
        "is_external": true,
        "line_number": null,
        "name": "reqwest",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "AiFixer 是一个智能Agent组件，用于自动修复Mermaid代码。它通过调用远程LLM（如Mistral）API，发送包含原始Mermaid代码的提示词，接收修复后的响应，并从响应中提取有效代码。组件支持JSON格式和Markdown代码块两种响应格式，具备自动清理和解析能力。它依赖外部配置（Config）获取API密钥、模型名称和基础URL，支持环境变量注入API密钥，具有良好的配置灵活性和容错机制。核心逻辑包括构建提示词、发送HTTP请求、处理API错误、解析响应内容（JSON或Markdown代码块）并返回修复后的代码。",
    "interfaces": [],
    "responsibilities": [
      "管理LLM API连接配置（API密钥、模型、基础URL）",
      "构建针对Mermaid代码修复的提示词模板",
      "与远程LLM服务进行HTTP通信并处理响应",
      "智能解析LLM返回的多种格式响应（JSON或Markdown代码块）",
      "提供错误处理和容错机制，确保服务稳定性"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "util",
      "description": null,
      "file_path": "src/utils.rs",
      "functions": [
        "extract_mermaid_blocks",
        "print_statistics"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "utils.rs",
      "source_summary": "/// 从markdown内容中提取mermaid代码块\n/// 返回 (开始位置, 结束位置, 代码内容) 的元组列表\npub fn extract_mermaid_blocks(content: &str) -> Vec<(usize, usize, String)> {\n    let mut blocks = Vec::new();\n    let lines: Vec<&str> = content.lines().collect();\n    let mut i = 0;\n\n    while i < lines.len() {\n        let line = lines[i].trim();\n        \n        // 查找mermaid代码块开始标记\n        if line == \"```mermaid\" {\n            let start_line = i;\n            i += 1;\n            let mut mermaid_lines = Vec::new();\n            \n            // 收集mermaid代码直到结束标记\n            while i < lines.len() {\n                let current_line = lines[i];\n                if current_line.trim() == \"```\" {\n                    // 找到结束标记\n                    let end_line = i;\n                    let mermaid_code = mermaid_lines.join(\"\\n\");\n                    \n                    // 计算在原始内容中的位置\n                    let start_pos = lines[..start_line].iter().map(|l| l.len() + 1).sum::<usize>();\n                    let end_pos = lines[..=end_line].iter().map(|l| l.len() + 1).sum::<usize>();\n                    \n                    blocks.push((start_pos, end_pos, mermaid_code));\n                    break;\n                }\n                mermaid_lines.push(current_line);\n                i += 1;\n            }\n        }\n        i += 1;\n    }\n\n    blocks\n}\n\n/// 打印统计信息\npub fn print_statistics(result: &crate::processor::ProcessResult, dry_run: bool) {\n    println!(\"\\n📊 处理完成:\");\n    println!(\"   📄 处理文件数: {}\", result.total_files);\n    println!(\"   📊 总mermaid代码块数: {}\", result.total_mermaid_blocks);\n    println!(\"   ❌ 无效代码块数: {}\", result.invalid_blocks);\n    if !dry_run {\n        println!(\"   🔧 成功修复数: {}\", result.fixed_blocks);\n    }\n}\n"
    },
    "complexity_metrics": {
      "cohesion_score": 0.95,
      "coupling_factor": 0.2,
      "cyclomatic_complexity": 6.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 51,
      "number_of_classes": 0,
      "number_of_functions": 2
    },
    "dependencies": [],
    "detailed_description": "该组件包含两个工具函数：extract_mermaid_blocks 用于从 Markdown 内容中提取 ```mermaid 代码块，返回每个代码块在原始文本中的起止位置和内容；print_statistics 用于格式化输出处理结果的统计信息，依赖外部的 ProcessResult 结构体。两个函数均为无状态工具函数，不维护任何内部状态，仅提供纯函数式数据处理与输出能力。",
    "interfaces": [],
    "responsibilities": [
      "从 Markdown 文本中解析并提取 Mermaid 代码块",
      "计算 Mermaid 代码块在原始文本中的字符级位置",
      "格式化输出处理流程的统计信息",
      "提供轻量级日志打印功能，支持干运行模式",
      "作为通用工具支持上游处理器的调试与反馈"
    ]
  },
  {
    "code_dossier": {
      "code_purpose": "entry",
      "description": null,
      "file_path": "src/cli.rs",
      "functions": [
        "Args::to_config"
      ],
      "importance_score": 0.8,
      "interfaces": [],
      "name": "cli.rs",
      "source_summary": "use clap::Parser;\nuse std::path::PathBuf;\n\nuse crate::config::Config;\n\n/// Mermaid Fixer - 基于Rust和AI的markdown文档修复器\n#[derive(Parser, Debug)]\n#[command(name = \"mermaid-fixer\")]\n#[command(about = \"基于Rust和AI的markdown文档修复器，用于扫描和修复markdown文件中的mermaid图表语法错误\")]\n#[command(author = \"jiangmeng03\")]\n#[command(version)]\npub struct Args {\n    /// 要扫描的目录路径\n    #[arg(short, long, value_name = \"DIR\")]\n    pub directory: PathBuf,\n\n    /// 配置文件路径\n    #[arg(short, long, value_name = \"FILE\", default_value = \"config.toml\")]\n    pub config: PathBuf,\n\n    /// 只检测问题，不进行修复\n    #[arg(long)]\n    pub dry_run: bool,\n\n    /// 启用详细日志输出\n    #[arg(short, long)]\n    pub verbose: bool,\n\n    /// LLM提供商 (openai, mistral, deepseek等)\n    #[arg(long)]\n    pub llm_provider: Option<String>,\n\n    /// LLM模型名称\n    #[arg(long)]\n    pub llm_model: Option<String>,\n\n    /// LLM API密钥\n    #[arg(long)]\n    pub llm_api_key: Option<String>,\n\n    /// LLM API基础URL\n    #[arg(long)]\n    pub llm_base_url: Option<String>,\n\n    /// 最大token数\n    #[arg(long)]\n    pub max_tokens: Option<u32>,\n\n    /// 温度参数 (0.0-1.0)\n    #[arg(long)]\n    pub temperature: Option<f32>,\n\n    /// Mermaid验证超时时间（秒）\n    #[arg(long)]\n    pub timeout_seconds: Option<u64>,\n\n    /// 最大重试次数\n    #[arg(long)]\n    pub max_retries: Option<u32>,\n}\n\nimpl Args {\n    /// 将CLI参数转换为配置\n    pub fn to_config(self) -> Result<Config, Box<dyn std::error::Error>> {\n        let mut config = if self.config.exists() {\n            Config::load(&self.config)?\n        } else {\n            Config::default()\n        };\n\n        // 覆盖LLM配置\n        if let Some(provider) = self.llm_provider {\n            config.llm.provider = provider;\n        }\n\n        if let Some(model) = self.llm_model {\n            config.llm.model = model;\n        }\n\n        if let Some(api_key) = self.llm_api_key {\n            config.llm.api_key = Some(api_key);\n        }\n\n        if let Some(base_url) = self.llm_base_url {\n            config.llm.base_url = Some(base_url);\n        }\n\n        if let Some(max_tokens) = self.max_tokens {\n            config.llm.max_tokens = Some(max_tokens);\n        }\n\n        if let Some(temperature) = self.temperature {\n            config.llm.temperature = Some(temperature);\n        }\n\n        // 覆盖Mermaid配置\n        if let Some(timeout) = self.timeout_seconds {\n            config.mermaid.timeout_seconds = Some(timeout);\n        }\n\n        if let Some(retries) = self.max_retries {\n            config.mermaid.max_retries = Some(retries);\n        }\n\n        Ok(config)\n    }\n}"
    },
    "complexity_metrics": {
      "cohesion_score": 0.9,
      "coupling_factor": 0.6,
      "cyclomatic_complexity": 10.0,
      "depth_of_inheritance": 0,
      "lines_of_code": 107,
      "number_of_classes": 1,
      "number_of_functions": 1
    },
    "dependencies": [
      {
        "dependency_type": "crate",
        "is_external": true,
        "line_number": null,
        "name": "clap",
        "path": null,
        "version": null
      },
      {
        "dependency_type": "module",
        "is_external": false,
        "line_number": null,
        "name": "Config",
        "path": "crate::config::Config",
        "version": null
      },
      {
        "dependency_type": "std",
        "is_external": true,
        "line_number": null,
        "name": "PathBuf",
        "path": null,
        "version": null
      }
    ],
    "detailed_description": "该组件是Mermaid Fixer项目的执行入口，基于clap库定义命令行参数解析结构体Args。它负责接收用户通过命令行传递的配置参数（如目录路径、LLM提供商、API密钥、超时时间等），并将其转换为内部Config配置对象。该结构体不直接执行文件扫描或修复逻辑，而是作为配置枢纽，将CLI输入映射到系统核心配置，供后续模块使用。其核心价值在于提供灵活、可扩展的命令行接口，支持覆盖默认配置和环境变量配置，实现配置优先级管理（CLI > 配置文件 > 默认值）。",
    "interfaces": [],
    "responsibilities": [
      "解析用户命令行参数",
      "将CLI参数转换为内部Config配置对象",
      "覆盖Config中的LLM和Mermaid相关配置项",
      "提供配置优先级管理机制（命令行 > 配置文件 > 默认值）",
      "封装配置加载逻辑，降低主程序耦合度"
    ]
  }
]
```

## Memory存储统计

**总存储大小**: 297629 bytes

- **documentation**: 181282 bytes (60.9%)
- **timing**: 36 bytes (0.0%)
- **preprocess**: 53482 bytes (18.0%)
- **studies_research**: 62829 bytes (21.1%)

## 生成文档统计

生成文档数量: 10 个

- 核心流程
- 项目概述
- 核心模块与组件调研报告_文件扫描域
- 核心模块与组件调研报告_AI修复域
- 核心模块与组件调研报告_配置管理域
- 核心模块与组件调研报告_CLI入口域
- 核心模块与组件调研报告_语法验证域
- 核心模块与组件调研报告_工具支持域
- 架构说明
- 核心模块与组件调研报告_处理协调域
