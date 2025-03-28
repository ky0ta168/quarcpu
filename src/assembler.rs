use std::collections::HashMap;

pub fn assemble(source: &str) -> Vec<u8> {
    let mut label_map = HashMap::new(); // ラベル -> アドレス
    let mut instructions = Vec::new(); // 命令を保存して2パス目に使う
    let mut byte_offset = 0; // 各命令の開始位置（バイト単位）

    // 1パス目 ラベルの調査
    for (i, line) in source.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if line.ends_with(':') {
            let label = line.trim_end_matches(':');
            label_map.insert(label.to_string(), byte_offset);
            continue;
        }

        let tokens: Vec<&str> = line
            .split_whitespace()
            .map(|s| s.trim_end_matches(','))
            .collect();

        instructions.push((tokens, i + 1)); // 命令と行番号を記録

        // 命令長をバイト単位で記録
        match instructions.last().unwrap().0[0] {
            "MOV" | "ADD" => byte_offset += 3,
            "PRINT" => byte_offset += 2,
            "JMP" => byte_offset += 2,
            "JZ" | "JNZ" => byte_offset += 3,
            "HALT" => byte_offset += 1,
            _ => panic!("Line {}: Unknown instruction", i + 1),
        }
    }

    // 2パス目 バイナリ命令列生成
    let mut output = Vec::new();

    for (tokens, line_num) in instructions {
        match tokens.as_slice() {
            ["MOV", reg, val] => {
                output.push(0x01);
                output.push(parse_reg(reg));
                output.push(parse_imm(val));
            }
            ["ADD", reg, val] => {
                output.push(0x02);
                output.push(parse_reg(reg));
                output.push(parse_imm(val));
            }
            ["PRINT", reg] => {
                output.push(0x03);
                output.push(parse_reg(reg));
            }
            ["JMP", label] => {
                output.push(0x10);
                let addr = resolve_label(label, &label_map, line_num);
                output.push(addr as u8);
            }
            ["JZ", reg, label] => {
                output.push(0x11);
                output.push(parse_reg(reg));
                let addr = resolve_label(label, &label_map, line_num);
                output.push(addr as u8);
            }
            ["JNZ", reg, label] => {
                output.push(0x12);
                output.push(parse_reg(reg));
                let addr = resolve_label(label, &label_map, line_num);
                output.push(addr as u8);
            }
            ["HALT"] => output.push(0xFF),
            _ => panic!("Line {}: Unknown instruction: {:?}", line_num, tokens),
        }
    }

    output
}

fn parse_reg(s: &str) -> u8 {
    match s {
        "R0" => 0,
        "R1" => 1,
        "R2" => 2,
        "R3" => 3,
        _ => panic!("Unknown register: {}", s),
    }
}

fn parse_imm(s: &str) -> u8 {
    let n: i8 = s.parse().expect("Invalid immediate");
    n as u8
}

fn resolve_label(label: &str, map: &HashMap<String, usize>, line: usize) -> usize {
    *map.get(label).unwrap_or_else(|| {
        panic!("Line {}: Undefined label '{}'", line, label);
    })
}
