mod compiler;
mod parser;
mod vm;
// use crate::compiler::{Op, Value};
use crate::parser::Expr;
use crate::vm::VM;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Использование: {} <путь_к_файлу>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    match std::fs::read_to_string(filename) {
        Ok(contents) => match parse_file_contents(&contents) {
            Ok(expr) => {
                compile(&expr);
            }
            Err(_) => {
                eprintln!("Ошибка при разборе файла {}", filename);
                std::process::exit(1)
            }
        },
        Err(e) => {
            eprintln!("Не удалось прочитать файл {}: {}", filename, e);
            std::process::exit(1);
        }
    }
}

fn parse_file_contents(contents: &String) -> Result<Expr, String> {
    match parser::parse(&contents) {
        Ok(expr) => {
            println!("Распарсенное выражение: {:?}", expr);
            Ok(expr.clone())
        }
        Err(e) => {
            eprintln!("Ошибка парсинга: {}", e);
            Err(e)
        }
    }
}

fn compile(expr: &Expr) {
    let (code, consts) = compiler::compile_expr(&expr);
    println!("Скомпилированный байт-код: {:?}", code);
    println!("Скомпилированные константы: {:?}", consts);

    let mut vm = VM::new(code, consts);
    let result = vm.run();
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::Value;
    #[test]
    fn parse_let_test() {
        let contents = "(let a (+ 2 3) (let b a (+ 5 b)))".to_string();
        let result = parse_file_contents(&contents).unwrap();
        assert_eq!(
            result,
            Expr::Let {
                name: "a".to_string(),
                value: Box::new(Expr::Add {
                    left: Box::new(Expr::Num(2)),
                    right: Box::new(Expr::Num(3))
                }),
                body: Box::new(Expr::Let {
                    name: "b".to_string(),
                    value: Box::new(Expr::Var("a".to_string())),
                    body: Box::new(Expr::Add {
                        left: Box::new(Expr::Num(5)),
                        right: Box::new(Expr::Var("b".to_string()))
                    })
                })
            }
        );
    }

    #[test]
    fn parse_if_test() {
        let contents = "(if 1 2 3)".to_string();
        let result = parse_file_contents(&contents).unwrap();
        assert_eq!(
            result,
            Expr::If {
                cond: Box::new(Expr::Num(1)),
                then: Box::new(Expr::Num(2)),
                else_: Box::new(Expr::Num(3))
            }
        );
    }

    #[test]
    fn parse_lam_test() {
        let contents = "(lambda x (+ x 1))".to_string();
        let result = parse_file_contents(&contents).unwrap();
        assert_eq!(
            result,
            Expr::Lam {
                param: "x".to_string(),
                body: Box::new(Expr::Add {
                    left: Box::new(Expr::Var("x".to_string())),
                    right: Box::new(Expr::Num(1))
                })
            }
        );
    }

    #[test]
    fn parse_let_if_test() {
        let contents = "(let x 10 (if x 20 30))".to_string();
        let result = parse_file_contents(&contents).unwrap();
        assert_eq!(
            result,
            Expr::Let {
                name: "x".to_string(),
                value: Box::new(Expr::Num(10)),
                body: Box::new(Expr::If {
                    cond: Box::new(Expr::Var("x".to_string())),
                    then: Box::new(Expr::Num(20)),
                    else_: Box::new(Expr::Num(30))
                })
            }
        );
    }

    fn parse_compile_and_run(contents: &String) -> Value {
        let expr = parse_file_contents(contents).unwrap();
        let (code, consts) = compiler::compile_expr(&expr);
        let mut vm = VM::new(code, consts);
        vm.run()
    }

    #[test]
    fn compile_and_run_let_test() {
        let contents = "(let a (+ 2 3) (let b a (+ 5 b)))".to_string();
        let result = parse_compile_and_run(&contents);
        assert_eq!(result, Value::Int(10));
    }

    #[test]
    fn compile_and_run_if_test() {
        let contents = "(let x 10 (if x 20 30))".to_string();
        let result = parse_compile_and_run(&contents);
        assert_eq!(result, Value::Int(20));
    }

    #[test]
    fn compile_and_run_lam_and_if_test() {
        let contents = "((lambda x (if x (+ 2 x) 100)) 5)".to_string();
        let result = parse_compile_and_run(&contents);
        assert_eq!(result, Value::Int(7));

        let contents = "((lambda x (if x (+ 2 x) 100)) 0)".to_string();
        let result = parse_compile_and_run(&contents);
        assert_eq!(result, Value::Int(100));
    }

    #[test]
    fn compile_and_run_lam_test() {
        let contents = "((lambda x (+ x 1)) 5)".to_string();
        let result = parse_compile_and_run(&contents);
        assert_eq!(result, Value::Int(6));
    }

    #[test]
    fn compile_and_run_lam_and_let_test() {
        let contents = "(let y (lambda x (+ x 1)) (y 41))".to_string();
        let result = parse_compile_and_run(&contents);
        assert_eq!(result, Value::Int(42));
    }

    #[test]
    fn compile_and_run_begin_test() {
        let contents = "
        (begin
            (define x 10)
            (define y (lambda z (+ 2 z)))
            (define v (y (y x)))
            (if (+ -10 v) (+ 1 v) (let z (+ 2 x) z)))"
            .to_string();
        let result = parse_compile_and_run(&contents);
        assert_eq!(result, Value::Int(15));
    }
}
