pub struct PeepholeOptimizer;

impl PeepholeOptimizer {
    pub fn optimize(asm: String) -> String {
        let lines: Vec<&str> = asm.lines().collect();
        let mut optimized = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let current = lines[i].trim();
            
            // 1. Remove redundant mov: mov [rbp-X], rax; mov rax, [rbp-X]
            if i + 1 < lines.len() {
                let next = lines[i+1].trim();
                if current.starts_with("mov [rbp") && current.ends_with("], rax") &&
                   next.starts_with("mov rax, [rbp") && next.ends_with("]") {
                    let addr1 = &current[4..current.len()-6];
                    let addr2 = &next[9..next.len()-1];
                    if addr1 == addr2 {
                        optimized.push(lines[i]);
                        i += 2; // Skip the redundant load
                        continue;
                    }
                }
            }

            // 2. Instruction Selection: mov rax, 0 -> xor eax, eax
            if current == "mov rax, 0" {
                optimized.push("  xor eax, eax");
                i += 1;
                continue;
            }

            // 3. Instruction Selection: add rax, 1 -> inc rax
            if current == "add rax, 1" {
                optimized.push("  inc rax");
                i += 1;
                continue;
            }
            if current == "sub rax, 1" {
                optimized.push("  dec rax");
                i += 1;
                continue;
            }

            optimized.push(lines[i]);
            i += 1;
        }

        optimized.join("\n")
    }
}
