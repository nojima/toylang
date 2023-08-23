mod ast;
mod token;
mod lexer;
mod eval;

use std::io::{self, Write};
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub syntax);

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        print!("expr> ");
        io::stdout().flush()?;

        buffer.clear();
        let n = stdin.read_line(&mut buffer)?;
        if n == 0 {
            return Ok(());
        }

        let lexer = lexer::Lexer::new(&buffer);
        let parser = syntax::ProgramParser::new();
        let expr = match parser.parse(lexer) {
            Ok(expr) => expr,
            Err(e) => {
                println!("ParseError: {e}");
                println!();
                continue;
            }
        };

        let env = eval::Environment::new();
        let value = match eval::eval(&env, &expr) {
            Ok(value) => value,
            Err(e) => {
                println!("EvalError: {e}");
                println!();
                continue;
            }
        };
        println!("=> {value}");
        println!();
    }
}

#[test]
fn parse_test() {
    let verify = |source: &str, expected: &str| {
        let lexer = lexer::Lexer::new(source);
        let parser = syntax::ProgramParser::new();
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
