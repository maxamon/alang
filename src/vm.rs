use crate::compiler::{Op, Value};

#[derive(Debug, Clone)]
struct Frame {
    ip: usize,
    code: Vec<Op>,
    locals: Vec<Value>,
}

pub struct VM {
    stack: Vec<Value>,
    frames: Vec<Frame>,
    consts: Vec<Value>,
    globals: Vec<Value>,
}

impl VM {
    pub fn new(code: Vec<Op>, consts: Vec<Value>) -> Self {
        println!(
            ">>>>>> Инициализация VM с кодом: {:?} и константами: {:?}",
            code, consts
        );
        VM {
            stack: vec![],
            frames: vec![Frame {
                ip: 0,
                code,
                locals: vec![],
            }],
            consts,
            globals: vec![],
        }
    }

    pub fn run(&mut self) -> Value {
        loop {
            let frame = self.frames.last_mut().unwrap();
            println!("==== Текущий фрейм: {:?}", frame);
            if frame.ip >= frame.code.len() {
                panic!("function fell off end without Ret");
            }
            let op = frame.code[frame.ip];
            frame.ip += 1;

            println!("Выполнение операции: {:?}", op);

            match op {
                Op::PushGlobal(idx) => {
                    self.stack.push(self.globals[idx].clone());
                }
                Op::StoreGlobal(idx) => {
                    let val = self.stack.pop().unwrap();
                    if idx >= self.globals.len() {
                        self.globals.resize(idx + 1, Value::Int(0));
                    }
                    self.globals[idx] = val;
                }
                Op::PushConst(idx) => {
                    self.stack.push(self.consts[idx].clone());
                }
                Op::PushLocal(idx) => {
                    self.stack.push(frame.locals[idx].clone());
                }
                Op::StoreLocal(idx) => {
                    let val = self.stack.pop().unwrap();
                    if idx >= frame.locals.len() {
                        frame.locals.resize(idx + 1, Value::Int(0));
                    }
                    frame.locals[idx] = val;
                }
                Op::MakeClosure(num_free) => {
                    let mut captured = Vec::new();
                    for _ in 0..num_free {
                        captured.push(self.stack.pop().unwrap());
                    }
                    let func = self.stack.pop().unwrap();
                    self.stack.push(Value::Closure {
                        func: Box::new(func),
                        captured,
                    });
                }
                Op::Call(argc) => {
                    println!(
                        "Вызов функции с {} аргументами и стеком {:?}",
                        argc, self.stack
                    );
                    let mut args = Vec::new();
                    for _ in 0..argc {
                        args.push(self.stack.pop().unwrap());
                    }
                    let callee = self.stack.pop().unwrap();
                    println!("Callee value: {:?} and args: {:?}", callee, args);

                    if let Value::Closure { func, captured } = callee {
                        if let Value::Func {
                            code,
                            arity,
                            num_free: _,
                            locals_size,
                        } = *func
                        {
                            if arity != argc as u8 {
                                panic!("arity mismatch: expected {}, got {}", arity, argc);
                            }
                            let mut locals = captured;
                            locals.extend(args);
                            locals.resize(locals_size, Value::Int(0));
                            println!("Preparing to call function with locals: {:?}", locals);
                            println!("Preparing to call function with consts: {:?}", self.consts);

                            // let old_frame = self.frames.last_mut().unwrap();
                            let old_ip = self.frames.last().unwrap().ip; // только & (не mut)
                                                                         // let old_code = self.frames.last().unwrap().code.clone(); // копия

                            self.frames.push(Frame {
                                ip: 0,
                                code,
                                locals,
                            });
                            let index = self.frames.len() - 2;
                            self.frames[index].ip = old_ip;
                        } else {
                            panic!("not a function");
                        }
                    } else {
                        panic!("not a closure");
                    }
                }
                Op::Ret => {
                    let result = self.stack.pop().unwrap();
                    self.frames.pop();
                    if self.frames.is_empty() {
                        return result;
                    }
                    self.stack.push(result);
                }
                Op::Add => {
                    let pop_value = self.stack.pop().unwrap();
                    println!("Popped value for addition: {:?}", pop_value);
                    let b = if let Value::Int(b) = pop_value {
                        b
                    } else {
                        panic!("You tried to add a non-integer value for B");
                    };
                    let a = if let Value::Int(a) = self.stack.pop().unwrap() {
                        a
                    } else {
                        panic!("You tried to add a non-integer value for A");
                    };
                    self.stack.push(Value::Int(a + b));
                }
                Op::Pop => {
                    let _ = self.stack.pop();
                }
                Op::JumpIfFalse(offset) => {
                    let cond = if let Value::Int(c) = self.stack.pop().unwrap() {
                        c
                    } else {
                        panic!()
                    };
                    if cond == 0 {
                        frame.ip = ((frame.ip as isize) + offset) as usize;
                    }
                }
                Op::Jump(offset) => {
                    frame.ip = ((frame.ip as isize) + offset) as usize;
                }
            }
            println!("Стек после операции: {:?}\n", self.stack);
        }
    }
}
