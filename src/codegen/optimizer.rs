pub struct PeepholeOptimizer;

impl PeepholeOptimizer {
    pub fn optimize(asm: String) -> String {
        let lines: Vec<&str> = asm.lines().collect();
        let mut optimized: Vec<String> = Vec::new();
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
                optimized.push("  xor eax, eax".to_string());
                rax_contents = Some("0".to_string());
                i += 1;
                continue;
            }
            if normalized == "add rax, 1" {
                optimized.push("  inc rax".to_string());
                rax_contents = None;
                i += 1;
                continue;
            }
            if normalized == "sub rax, 1" {
                optimized.push("  dec rax".to_string());
                rax_contents = None;
                i += 1;
                continue;
            }

            // 4. Strength Reduction
            if normalized == "imul rax, 2" { optimized.push("  shl rax, 1".to_string()); rax_contents = None; i += 1; continue; }
            if normalized == "imul rax, 4" { optimized.push("  shl rax, 2".to_string()); rax_contents = None; i += 1; continue; }
            if normalized == "imul rax, 8" { optimized.push("  shl rax, 3".to_string()); rax_contents = None; i += 1; continue; }

            // 5. Condition Code Fusion (setCC + jmp)
            if normalized.starts_with("set") && normalized.ends_with(" al") && i + 4 < lines.len() {
                let cc = &normalized[3..normalized.len()-3]; // e.g., 'e', 'l', 'g', 'le', 'ge', 'ne'
                let next1 = lines[i+1].trim().to_string();
                let next2 = lines[i+2].trim().to_string();
                let next3 = lines[i+3].trim().to_string();
                let next4 = lines[i+4].trim().to_string();

                if next1 == "movzx rax, al" && next2.starts_with("mov [rbp-") && next3 == "cmp rax, 0" {
                    if next4.starts_with("je ") || next4.starts_with("jne ") {
                        let is_je = next4.starts_with("je ");
                        let target = if is_je { &next4[3..] } else { &next4[4..] };
                        
                        let final_cc = match (cc, is_je) {
                            ("e", false) => "je", ("e", true) => "jne",
                            ("ne", false) => "jne", ("ne", true) => "je",
                            ("l", false) => "jl", ("l", true) => "jge",
                            ("le", false) => "jle", ("le", true) => "jg",
                            ("g", false) => "jg", ("g", true) => "jle",
                            ("ge", false) => "jge", ("ge", true) => "jl",
                            _ => "",
                        };

                        if !final_cc.is_empty() {
                            optimized.push(format!("  {} {}", final_cc, target));
                            i += 5;
                            rax_contents = None;
                            continue;
                        }
                    }
                }
            }

            // 6. Jump to next line
            if normalized.starts_with("jmp .") && i + 1 < lines.len() {
                let target = &normalized[4..];
                let next_line = lines[i+1].trim();
                if next_line.starts_with(target) && next_line.ends_with(":") {
                    i += 1;
                    continue;
                }
            }

            optimized.push(line.to_string());
            i += 1;
        }

        // Pass 2: Identify read stack slots
        let mut read_slots = std::collections::HashSet::new();
        for line in &optimized {
            let s = line.trim();
            let mut start = 0;
            while let Some(idx) = s[start..].find("[rbp-") {
                let abs_idx = start + idx;
                if let Some(end_idx) = s[abs_idx..].find(']') {
                    let slot = &s[abs_idx..abs_idx+end_idx+1];
                    let is_write = s.starts_with("mov ") && s[4..].trim_start().starts_with(slot) && !s.contains("qword");
                    let is_write_qword = s.starts_with("mov qword ") && s[10..].trim_start().starts_with(slot);
                    
                    if !(is_write || is_write_qword) {
                        read_slots.insert(slot.to_string());
                    }
                    start = abs_idx + end_idx + 1;
                } else {
                    break;
                }
            }
        }

        // Pass 3: Remove dead writes
        let mut final_asm = Vec::new();
        for line in optimized {
            let s = line.trim();
            let mut is_dead_write = false;
            
            if s.starts_with("mov ") {
                let dest_part = s.replace("mov qword ", "").replace("mov ", "");
                let dest_part = dest_part.trim_start();
                if dest_part.starts_with("[rbp-") {
                    if let Some(end_idx) = dest_part.find(']') {
                        let slot = &dest_part[..end_idx+1];
                        if !read_slots.contains(slot) {
                            is_dead_write = true;
                        }
                    }
                }
            }
            
            if !is_dead_write {
                final_asm.push(line);
            }
        }

        final_asm.join("\n")
    }
}
