# quarcpu

## 概要

自作の仮想CPUを作成。

小さいCPUをイメージして命名。

Quark + CPU → **QuarCPU**

## 命令セット

| opcode | 命令 | 意味 |
| ------ | --- | --- |
| `0x01` | MOV | `MOV Rn, imm` → Rnに即値を格納 |
| `0x02` | ADD | `ADD Rn, imm` → Rnに即値を加算 |
| `0x03` | PRINT | `PRINT Rn` → Rnの値を出力 |
| `0xFF` | HALT | プログラム終了 |
| `0x10` | JMP | `JMP addr` → 無条件ジャンプ（ip を addr にセット） |
| `0x11` | JZ | `JZ reg, addr` → reg が 0 ならジャンプ |
| `0x12` | JNZ | `JNZ reg, addr` → reg が 0 でなければジャンプ |

## Demo

```
cd quarcpu
cargo run -- examples/program.asm
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/quarcpu examples/program.asm`
5
4
3
2
1
10
8
6
4
2
0
```