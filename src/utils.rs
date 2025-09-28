/// 从markdown内容中提取mermaid代码块
/// 返回 (开始位置, 结束位置, 代码内容) 的元组列表
pub fn extract_mermaid_blocks(content: &str) -> Vec<(usize, usize, String)> {
    let mut blocks = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();
        
        // 查找mermaid代码块开始标记
        if line == "```mermaid" {
            let start_line = i;
            i += 1;
            let mut mermaid_lines = Vec::new();
            
            // 收集mermaid代码直到结束标记
            while i < lines.len() {
                let current_line = lines[i];
                if current_line.trim() == "```" {
                    // 找到结束标记
                    let end_line = i;
                    let mermaid_code = mermaid_lines.join("\n");
                    
                    // 计算在原始内容中的位置
                    let start_pos = lines[..start_line].iter().map(|l| l.len() + 1).sum::<usize>();
                    let end_pos = lines[..=end_line].iter().map(|l| l.len() + 1).sum::<usize>();
                    
                    blocks.push((start_pos, end_pos, mermaid_code));
                    break;
                }
                mermaid_lines.push(current_line);
                i += 1;
            }
        }
        i += 1;
    }

    blocks
}

/// 打印统计信息
pub fn print_statistics(result: &crate::processor::ProcessResult, dry_run: bool) {
    println!("\n📊 处理完成:");
    println!("   📄 处理文件数: {}", result.total_files);
    println!("   📊 总mermaid代码块数: {}", result.total_mermaid_blocks);
    println!("   ❌ 无效代码块数: {}", result.invalid_blocks);
    if !dry_run {
        println!("   🔧 成功修复数: {}", result.fixed_blocks);
    }
}
