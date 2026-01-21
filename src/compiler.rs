use std::collections::HashMap;
use std::collections::HashSet;

use crate::parser::Expr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    PushConst(usize),
    PushLocal(usize),
    PushGlobal(usize),
    StoreLocal(usize),
    StoreGlobal(usize),
    Call(u8),
    Ret,
    Add,
    JumpIfFalse(isize),
    Jump(isize),
    MakeClosure(u8),
    Pop,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Func {
        code: Vec<Op>,
        arity: u8,
        num_free: u8,
        locals_size: usize,
    },
    Closure {
        func: Box<Value>,
        captured: Vec<Value>,
    },
}

struct Compiler {
    code: Vec<Op>,
    consts: Vec<Value>,
    locals: Vec<String>,
    globals: Vec<Value>,
    global_map: HashMap<String, usize>,
}

impl Compiler {
    fn new() -> Self {
        Compiler {
            code: vec![],
            consts: vec![],
            locals: vec![],
            globals: vec![],
            global_map: HashMap::new(),
        }
    }

    fn emit(&mut self, op: Op) {
        self.code.push(op);
        println!(
            "Эмиттированная операция: {:?} {:?} {:?} {:?}",
            op, self.code, self.consts, self.locals
        );
    }

    fn add_const(&mut self, val: Value) -> usize {
        let idx = self.consts.len();
        self.consts.push(val);
        idx
    }

    fn compile(&mut self, expr: &Expr) {
        println!("Компиляция выражения: {:?}", expr);
        match expr {
            Expr::Num(n) => {
                println!("Компиляция числа: {:?}", n);
                let idx = self.add_const(Value::Int(*n));
                self.emit(Op::PushConst(idx));
            }
            Expr::Var(name) => {
                println!("Компиляция переменной: {:?}", self.code);
                // if let Some(idx) = self.locals.iter().rposition(|n| n == name) {
                //     self.emit(Op::PushLocal(idx));
                // } else {
                //     panic!("Undefined variable: {}", name);
                // }
                if let Some(&idx) = self.global_map.get(name) {
                    self.emit(Op::PushGlobal(idx));
                } else if let Some(idx) = self.locals.iter().rposition(|s| s == name) {
                    self.emit(Op::PushLocal(idx));
                } else {
                    panic!("undefined: {}", name);
                }
            }
            Expr::Lam { param, body } => {
                println!(
                    "Компиляция лямбды с параметром: {:?} и body: {:?}",
                    param, body
                );
                let mut fv_set = free_vars(body);
                fv_set.remove(param);
                println!(
                    "Свободные переменные для лямбды {:?}, {:?}",
                    fv_set, self.locals
                );
                let mut fv: Vec<String> = fv_set.into_iter().collect();
                fv.sort();

                for name in &fv {
                    if let Some(idx) = self.locals.iter().rposition(|n| n == name) {
                        self.emit(Op::PushLocal(idx));
                    } else {
                        panic!("Undefined variable in closure: {}", name);
                    }
                }

                let mut sub = Compiler::new();
                sub.locals = fv.clone();
                sub.locals.push(param.clone());

                sub.compile(body);
                sub.emit(Op::Ret);

                // Перенос констант
                let offset = self.consts.len();
                for c in sub.consts {
                    self.add_const(c);
                }

                // Пересчёт индексов в code (только PushConst)
                for op in &mut sub.code {
                    if let Op::PushConst(idx) = op {
                        *idx += offset;
                    }
                }

                let func = Value::Func {
                    code: sub.code,
                    arity: 1,
                    num_free: fv.len() as u8,
                    locals_size: sub.locals.len(),
                };
                let idx = self.add_const(func);
                self.emit(Op::PushConst(idx));
                self.emit(Op::MakeClosure(fv.len() as u8));
            }
            Expr::App { func, args } => {
                println!("Функция и аргументы для вызова: {:?} {:?}", func, args);
                self.compile(func);
                for arg in args {
                    self.compile(arg);
                }
                self.emit(Op::Call(args.len() as u8));
            }
            Expr::Let { name, value, body } => {
                self.compile(value);
                let idx = self.locals.len();
                self.emit(Op::StoreLocal(idx));
                self.locals.push(name.clone());
                self.compile(body);
                self.locals.pop();
            }
            Expr::Define { name, value } => {
                self.compile(value);
                let idx = self.globals.len();
                self.globals.push(Value::Int(0)); // placeholder
                self.emit(Op::StoreGlobal(idx));
                self.global_map.insert(name.clone(), idx);
            }
            Expr::If { cond, then, else_ } => {
                self.compile(cond);
                let jump_if_false_pos = self.code.len();
                self.emit(Op::JumpIfFalse(0)); // Placeholder

                self.compile(then);
                let jump_pos = self.code.len();
                self.emit(Op::Jump(0)); // Placeholder

                let false_target = self.code.len() as isize;
                self.code[jump_if_false_pos] =
                    Op::JumpIfFalse(false_target - jump_if_false_pos as isize - 1);

                self.compile(else_);
                let end = self.code.len() as isize;
                self.code[jump_pos] = Op::Jump(end - jump_pos as isize - 1);
            }
            Expr::Add { left, right } => {
                self.compile(left);
                self.compile(right);
                self.emit(Op::Add);
            }
            Expr::Begin(exprs) => {
                if exprs.is_empty() {
                    let idx = self.add_const(Value::Int(0));
                    self.emit(Op::PushConst(idx));
                    return;
                }

                for (i, e) in exprs.iter().enumerate() {
                    println!("Компиляция Begin, выражение: {:?}", e);
                    self.compile(e);
                    if i < exprs.len() - 1 {
                        self.emit(Op::Pop);
                    }
                }
            }
        }
    }

    fn finish(mut self) -> (Vec<Op>, Vec<Value>) {
        self.emit(Op::Ret);
        println!(
            "Завершенный код: {:?}, константы: {:?}",
            self.code, self.consts
        );
        (self.code, self.consts)
    }
}

fn free_vars(expr: &Expr) -> HashSet<String> {
    println!("Вычисление свободных переменных для выражения: {:?}", expr);
    match expr {
        Expr::Num(_) => HashSet::new(),
        Expr::Var(name) => [name.clone()].into_iter().collect(),
        Expr::Lam { param, body } => {
            println!("Вычисление свободных переменных для тела лямбды");
            let mut vars = free_vars(body);
            println!("free vars of body: {:?}", vars);
            vars.remove(param);
            println!("after removing param {:?}, vars: {:?}", param, vars);
            vars
        }
        Expr::App { func, args } => {
            let mut vars = free_vars(func);
            for arg in args {
                vars.extend(free_vars(arg));
            }
            vars
        }
        Expr::Let { name, value, body } => {
            let mut vars = free_vars(body);
            println!("free vars of body: {:?}", vars);
            vars.extend(free_vars(value));
            vars.remove(name);
            vars
        }
        Expr::If { cond, then, else_ } => {
            let mut vars = free_vars(cond);
            vars.extend(free_vars(then));
            vars.extend(free_vars(else_));
            vars
        }
        Expr::Add { left, right } => {
            let mut vars = free_vars(left);
            vars.extend(free_vars(right));
            vars
        }
        Expr::Begin(exprs) => {
            println!("Вычисление свободных переменных для Begin");
            let mut vars = HashSet::new();
            for expr in exprs {
                println!(
                    "Вычисление свободных переменных для выражения в Begin: {:?}",
                    expr
                );
                vars.extend(free_vars(expr));
            }
            vars
        }
        Expr::Define { name, value } => {
            let mut fv = free_vars(value);
            fv.remove(name);
            fv
        }
    }
}

pub fn compile_expr(expr: &Expr) -> (Vec<Op>, Vec<Value>) {
    let mut compiler = Compiler::new();
    compiler.compile(expr);
    compiler.finish()
}
