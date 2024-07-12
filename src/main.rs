use std::env;
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct PackageJson {
    scripts: Scripts,
}

type Scripts = std::collections::HashMap<String, String>;

fn exec(command: &str) {
    println!("   > EXEC: {}", command);

    use std::process::Command;
    let mut parts = command.split_whitespace();
    let command = parts.next().unwrap();
    let mut args = parts;

    // cdコマンドの場合set_current_dirを使う
    if command == "cd" {
        let dir = args.next().unwrap();
        std::env::set_current_dir(dir).expect("Failed to change directory");
        return;
    }

    let status = Command::new(command)
        .args(args)
        .status()
        .expect("Failed to execute command");
    if !status.success() {
        eprintln!("Command failed with exit code: {}", status);
        std::process::exit(1);
    }
}

// &&で連結したコマンドを分割して実行する
fn exec_commands(commands: &str) {
    println!("Running commands: {}", commands);

    for command in commands.split("&&") {
        exec(command.trim());
    }
}

fn main() {
    const FILENAME: &str = "package.json";

    // ファイルを開いてJSONをパースする
    let file = File::open(FILENAME).expect("Failed to open file");
    let reader = BufReader::new(file);
    let package_json: PackageJson = serde_json::from_reader(reader).expect("Failed to parse JSON");

    // コマンドライン引数から指定されたスクリプト名を取得
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let script_name = &args[1]; // 第2引数をスクリプト名とする

    // スクリプトを実行
    match package_json.scripts.get(script_name) {
        Some(script) => exec_commands(script),
        None => std::process::exit(1),
    }
}
