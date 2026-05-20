pub struct PeepholeOptimizer;

impl PeepholeOptimizer {
    pub fn optimize(asm: String) -> String {
        let lines: Vec<&str> = asm.lines().collect();
        let mut optimized = Vec::new();
        let mut rax_contents: Option<String> = None;
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];
            let current = line.trim().to_string();
            
            // Normalize "mov rax, qword [..." to "mov rax, [..." for comparison
            let normalized = current.replace("qword ", "");

            // 1. Redundant load tracking
            if normalized.starts_with("mov rax, ") {
                let val = normalized[8..].trim().trim_start_matches(',').trim().to_string();
                if let Some(ref rax_val) = rax_contents {
                    if rax_val == &val {
                        i += 1;
                        continue;
                    }
                }
                rax_contents = Some(val);
            } else if normalized.starts_with("mov [rbp") && normalized.ends_with("], rax") {
                let parts: Vec<&str> = normalized.split(',').collect();
                if let Some(dest) = parts.first() {
                    let addr = dest[4..].trim().to_string(); 
                    rax_contents = Some(addr);
                }
            } else if normalized.starts_with("add rax, ") || normalized.starts_with("sub rax, ") || 
                      normalized.starts_with("imul rax, ") || normalized.starts_with("idiv ") ||
                      normalized.starts_with("xor eax, eax") || normalized.starts_with("shl rax, ") ||
                      normalized.starts_with("inc rax") || normalized.starts_with("dec rax") ||
                      normalized.starts_with("neg rax") || normalized.starts_with("movzx rax, ") ||
                      normalized.starts_with("lea rax, ") {
                rax_contents = None; // rax changed
            } else if normalized.starts_with("call ") {
                rax_contents = None; // rax changed by call (return value)
            }

            // 2. Algebraic Simplification: add rax, 0 -> remove
            if normalized == "add rax, 0" || normalized == "sub rax, 0" {
                i += 1;
                continue;
            }

            // 3. Instruction Selection
            if normalized == "mov rax, 0" {
                optimized.push("  xor eax, eax");
                rax_contents = Some("0".to_string());
                i += 1;
                continue;
            }
            if normalized == "add rax, 1" {
                optimized.push("  inc rax");
                rax_contents = None;
                i += 1;
                continue;
            }
            if normalized == "sub rax, 1" {
                optimized.push("  dec rax");
                rax_contents = None;
                i += 1;
                continue;
            }

            // 4. Strength Reduction
            if normalized == "imul rax, 2" { optimized.push("  shl rax, 1"); rax_contents = None; i += 1; continue; }
            if normalized == "imul rax, 4" { optimized.push("  shl rax, 2"); rax_contents = None; i += 1; continue; }
            if normalized == "imul rax, 8" { optimized.push("  shl rax, 3"); rax_contents = None; i += 1; continue; }

            // 5. Jump to next line
            if normalized.starts_with("jmp .") && i + 1 < lines.len() {
                let target = &normalized[4..];
                let next_line = lines[i+1].trim();
                if next_line.starts_with(target) && next_line.ends_with(":") {
                    i += 1;
                    continue;
                }
            }

            optimized.push(line);
            i += 1;
        }

        optimized.join("\n")
    }
}
