pub fn assemble(source: &str) -> Vec<u8> {
    let mut output = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            // 空行・コメント行はスキップ
            continue;
        }

        let tokens: Vec<&str> = line
            .split_whitespace()
            .map(|s| s.trim_end_matches(',')) // "R0,"となるので","を削除
            .collect();
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
            ["JMP", addr] => {
                output.push(0x10);
                output.push(parse_imm(addr));
            }
            ["JZ", reg, addr] => {
                output.push(0x11);
                output.push(parse_reg(reg));
                output.push(parse_imm(addr));
            }
            ["JNZ", reg, addr] => {
                output.push(0x12);
                output.push(parse_reg(reg));
                output.push(parse_imm(addr));
            }
            ["HALT"] => {
                output.push(0xFF);
            }
            _ => panic!("Line {}: Unknown instruction: {:?}", i + 1, line),
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
