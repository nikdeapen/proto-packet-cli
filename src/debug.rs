use lex::{Lex, Lexer};

/// Runs the lex command.
pub async fn lex(_path: &str, args: &[String]) -> Result<(), String> {
    if args.len() == 0 {
        Err("no file arg given".to_string())
    } else {
        let file: &str = &args[0];
        println!("Lexing: {}", file);

        if args.len() > 1 {
            Err(format!("lex unknown args: {:?}", &args[1..]))
        } else {
            let result: tokio::io::Result<String> = tokio::fs::read_to_string(file).await;
            match result {
                Err(error) => Err(error.to_string()),
                Ok(contents) => {
                    let result: Result<Lex, &'static str> = Lexer::default().lex(contents.as_str());
                    match result {
                        Err(error) => Err(error.to_string()),
                        Ok(lex) => {
                            println!("{}", lex);
                            Ok(())
                        }
                    }
                }
            }
        }
    }
}
