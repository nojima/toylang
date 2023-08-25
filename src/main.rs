mod ast;
mod eval;
mod lexer;
mod token;
mod value;

use clap::Parser;
use lalrpop_util::lalrpop_mod;
use std::path::{Path, PathBuf};

lalrpop_mod!(pub syntax);

#[derive(Parser, Debug)]
struct Cli {
    filename: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.filename {
        Some(filename) => execute_file(&filename),
        None => repl(),
    }
}

fn execute_file(filename: &Path) -> anyhow::Result<()> {
    let source_code = std::fs::read_to_string(filename)?;
    let lexer = lexer::Lexer::new(&source_code);
    let parser = syntax::ProgramParser::new();
    let node = parser.parse(lexer)?;
    let env = eval::Environment::new();
    let (value, _) = eval::eval_program(&env, &node)?;
    println!("{value:?}");
    Ok(())
}

fn repl() -> anyhow::Result<()> {
    let mut rl = rustyline::DefaultEditor::new()?;
    let mut env = eval::Environment::new();
    loop {
        let line = rl.readline(">> ")?;

        let lexer = lexer::Lexer::new(&line);
        let parser = syntax::ProgramParser::new();
        let node = match parser.parse(lexer) {
            Ok(node) => node,
            Err(e) => {
                println!("ParseError: {e}");
                println!();
                continue;
            }
        };

        let (value, new_env) = match eval::eval_program(&env, &node) {
            Ok(value) => value,
            Err(e) => {
                println!("EvalError: {e}");
                println!();
                continue;
            }
        };
        env = new_env;
        println!("=> {value:?}");
        println!();
    }
}

#[test]
fn parse_expr() {
    let verify = |source: &str, expected: &str| {
        let lexer = lexer::Lexer::new(source);
        let parser = syntax::ExprParser::new();
        let maybe_ast = parser.parse(lexer);
        let actual = maybe_ast.map(|ast| format!("{:?}", ast));
        assert_eq!(actual, Ok(expected.to_owned()));
    };

    verify("0", "0.0");
    verify("1", "1.0");
    verify("10", "10.0");
    verify("-1", "-1.0");
    verify("3.14", "3.14");
    verify("-3.14", "-3.14");
    verify("1e10", "10000000000.0");
    verify("1E10", "10000000000.0");
    verify("-1E10", "-10000000000.0");

    verify("1 + 2", "(1.0 + 2.0)");
    verify("1 + 2 * 3 / 4", "(1.0 + ((2.0 * 3.0) / 4.0))");
    verify("3 + -(1 - 2)", "(3.0 + -(1.0 - 2.0))");

    verify("foo", "foo");
    verify("foo * bar", "(foo * bar)");

    verify(
        "let foo = 1 + 2 in foo * 3",
        "(let foo = (1.0 + 2.0) in (foo * 3.0))",
    );
}
