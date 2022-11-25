use std::env;

use tokio;

pub mod debug;

/// ProtoPacket CLI Main
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        eprintln!("empty args");
        return;
    }

    let path: &str = args[0].as_str();
    if args.len() == 1 {
        eprintln!("no command arg: path={}", path);
        return;
    }

    let command: &str = args[1].as_str();
    let result: Result<(), String> = run(path, command, &args[2..]).await;
    if let Err(error) = result {
        eprintln!("{}", error);
    }
}

/// Runs the command.
async fn run(path: &str, command: &str, args: &[String]) -> Result<(), String> {
    match command {
        "lex" => debug::lex(path, args).await,
        _ => Err(format!("unknown command: {}", command)),
    }
}
