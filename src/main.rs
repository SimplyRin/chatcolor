use std::env;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let input = if !args.is_empty() {
        args.join(" ")
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
    };

    println!("{}", translate_colors(&input));
}

fn translate_colors(text: &str) -> String {
    let reset = "\x1b[0m";

    // 16色 (従来カラー)
    let colors = [
        ('0', "\x1b[30m"), // 黒
        ('1', "\x1b[34m"), // 濃青
        ('2', "\x1b[32m"), // 濃緑
        ('3', "\x1b[36m"), // 青緑
        ('4', "\x1b[31m"), // 濃赤
        ('5', "\x1b[35m"), // 紫
        ('6', "\x1b[33m"), // 金
        ('7', "\x1b[37m"), // 灰色
        ('8', "\x1b[90m"), // 濃灰
        ('9', "\x1b[94m"), // 明るい青
        ('a', "\x1b[92m"), // 明るい緑
        ('b', "\x1b[96m"), // 明るい水色
        ('c', "\x1b[91m"), // 明るい赤
        ('d', "\x1b[95m"), // 明るい紫
        ('e', "\x1b[93m"), // 明るい黄
        ('f', "\x1b[97m"), // 白
    ];

    // 装飾コード
    let formats = [
        ('k', ""),          // obfuscated → 未対応
        ('l', "\x1b[1m"),   // 太字
        ('m', "\x1b[9m"),   // 取り消し線
        ('n', "\x1b[4m"),   // 下線
        ('o', "\x1b[3m"),   // イタリック
        ('r', reset),       // リセット
    ];

    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if (ch == '§' || ch == '&') && chars.peek().is_some() {
            let code = chars.peek().unwrap().to_ascii_lowercase();

            // RGBカラー対応 (例: §x§f§f§a§a§c§c → #ffaacc)
            if code == 'x' {
                chars.next();
                let mut hex = String::new();
                for _ in 0..6 {
                    if chars.next() == Some('§') {
                        if let Some(h) = chars.next() {
                            hex.push(h);
                        }
                    }
                }
                if hex.len() == 6 {
                    if let Ok(rgb) = u32::from_str_radix(&hex, 16) {
                        let r = (rgb >> 16) & 0xFF;
                        let g = (rgb >> 8) & 0xFF;
                        let b = rgb & 0xFF;
                        result.push_str(&format!("\x1b[38;2;{};{};{}m", r, g, b));
                        continue;
                    }
                }
            }

            let code = chars.next().unwrap().to_ascii_lowercase();
            if let Some((_, ansi)) = colors.iter().find(|(c, _)| *c == code) {
                result.push_str(ansi);
                continue;
            }
            if let Some((_, ansi)) = formats.iter().find(|(c, _)| *c == code) {
                result.push_str(ansi);
                continue;
            }
        }
        result.push(ch);
    }

    result.push_str(reset);
    result
}
