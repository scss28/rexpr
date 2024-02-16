use std::{
    error,
    io::{self, Write},
};

use clap::Parser;

mod binary_operator;
mod expression_tree;
mod parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// A simple app used to evaluate mathematical expressions.
/// Supported operators: +, -, *, /, ** (power), %, ^, |, &
struct Args {
    /// Expression to be evaluated (optional)
    expression: Option<String>,
    /// Display output without colors
    #[arg(short, long, default_value_t = false)]
    boring: bool,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();
    if let Some(expression) = args.expression {
        match parser::parse_expression(expression.as_bytes()) {
            Ok(ast) => println!("{}", ast.evaluate()),
            Err(err) => {
                std::io::stderr()
                    .write_all(format!("{} (at index = {})", err.1, err.0).as_bytes())?;
            }
        }

        return Ok(());
    }

    println!(
        "{}",
        if args.boring {
            "| Input a math expression (\"q\" to exit)."
        } else {
            "| \x1b[90mInput a math expression (\"q\" to exit).\x1b[0m"
        }
    );

    loop {
        print!(
            "{}",
            if args.boring {
                "| > "
            } else {
                "\x1b[96m| > \x1b[0m"
            }
        );
        io::stdout().flush()?;

        let expr = {
            let mut expr = String::new();
            io::stdin().read_line(&mut expr)?;
            expr.trim_end().to_owned()
        };

        if expr == "q" {
            break;
        }

        match parser::parse_expression(expr.as_bytes()) {
            Ok(ast) => {
                #[cfg(debug_assertions)]
                {
                    println!("\x1b[94m|\x1b[0m {:?}", ast);
                }

                println!(
                    "{}",
                    if args.boring {
                        format!("| = {}\n", ast.evaluate())
                    } else {
                        format!("\x1b[94m| =\x1b[0m {}\n", ast.evaluate())
                    }
                )
            }
            Err(err) => {
                print!("{}", if args.boring { "    " } else { "\x1b[93m    " });
                for _ in 0..err.0 {
                    print!(" ");
                }

                for _ in 0..(expr.len() - err.0) {
                    print!("^");
                }

                println!(
                    "{}",
                    if args.boring {
                        format!("\n| {}", err.1)
                    } else {
                        format!("\n|\x1b[0m {}", err.1)
                    }
                );
            }
        }
    }

    Ok(())
}
