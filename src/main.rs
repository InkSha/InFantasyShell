use std::io::{Read, Write};

fn get_prompt() -> String {
    "INFS> ".to_string()
}

const QUIT_COMMAND: &str = "exit";

fn main() {
    print!("\x1B[2J\x1B[1;1H");

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buffer = Vec::new();

    print!("{}", get_prompt());
    stdout.flush().unwrap();

    for byte in stdin.lock().bytes() {
        let b = byte.expect("读取失败");

        if b == b'\r' {
            continue; // 跳过 Windows 下的 '\r'
        }

        if b == b'\n' {
            let input = String::from_utf8_lossy(&buffer).trim().to_string();
            if input == QUIT_COMMAND {
                println!("\nBye!");
                break;
            }
            println!(); // 换行
            buffer.clear();
            print!("{}", get_prompt());
        } else {
            buffer.push(b);
            stdout.write(&[b]).unwrap();
        }

        stdout.flush().unwrap();
    }
}
