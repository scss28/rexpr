use std::{
    env, error,
    io::{self, Write},
};

mod ast;
mod operator;
mod parser;

fn main() -> Result<(), Box<dyn error::Error>> {
    if env::args().len() > 1 {
        let expression = env::args().skip(1).map(|str| str + " ").collect::<String>();
        match parser::parse_expression(&expression) {
            Ok(ast) => println!("\x1b[94m| =\x1b[0m {}\n", ast.evaluate()),
            Err(err) => {
                for (i, char) in expression.chars().enumerate() {
                    if i == err.0 {
                        print!("\x1b[93m");
                    }

                    print!("{char}");
                }

                println!();
                for _ in 0..err.0 {
                    print!(" ");
                }

                for _ in 0..(expression.len() - err.0) {
                    print!("^");
                }

                println!("\n|\x1b[0m {}", err.1);
            }
        }

        return Ok(());
    }

    println!("| \x1b[90mInput a math expression (q -> exit).\x1b[0m");
    loop {
        print!("\x1b[96m| > \x1b[0m");
        io::stdout().flush()?;

        let expr = {
            let mut expr = String::new();
            io::stdin().read_line(&mut expr)?;
            expr.trim_end().to_owned()
        };

        if expr == "q" {
            break;
        }

        match parser::parse_expression(&expr) {
            Ok(ast) => {
                #[cfg(debug_assertions)]
                {
                    println!("\x1b[94m|\x1b[0m {:?}", ast);
                }

                println!("\x1b[94m| =\x1b[0m {}\n", ast.evaluate())
            }
            Err(err) => {
                print!("\x1b[93m    ");
                for _ in 0..err.0 {
                    print!(" ");
                }

                for _ in 0..(expr.len() - err.0) {
                    print!("^");
                }

                println!("\n|\x1b[0m {}", err.1);
            }
        }
    }

    Ok(())
}
