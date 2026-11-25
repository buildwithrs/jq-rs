use std::{env, fs::File, io::Read};

use jq_rs::{
    filter::apply_filter,
    json::{Lexer, Parser},
};

fn main() -> anyhow::Result<()> {
    println!("Welcome to the live session for building jq(json query) in Rust");

    let args: Vec<String> = env::args().collect();
    let mut json_data = String::new();
    let filter = &args[1];
    if filter.is_empty() {
        eprintln!("need filter");
        return Ok(());
    }

    File::open(&args[2])?.read_to_string(&mut json_data)?;

    // let input = r#"{"name": "alice", "age": 24, "is_programmer": true, "with_AI": false, "gender": null, "hobby": ["programming", "writing"]}"#;
    let mut lexer = Lexer::new(&json_data);
    // println!("{:?}", lexer);

    let tokens = lexer.tokenize();
    match tokens {
        Ok(tks) => {
            //println!("tokens: {:?}", tks);
            let mut parser = Parser::new(tks);
            match parser.parse_value() {
                Ok(v) => {
                    println!("success parsed json");
                    let res = apply_filter(filter, v);
                    println!("result: {:?}", res);
                }
                Err(e) => eprintln!("faield to parse: {e}"),
            }
        }
        Err(e) => eprintln!("failed to tokenize: {e}"),
    }
    Ok(())
}
