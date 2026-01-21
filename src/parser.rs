use std::vec;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(i64),
    Var(String),
    Lam {
        param: String,
        body: Box<Expr>,
    },
    App {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    Let {
        name: String,
        value: Box<Expr>,
        body: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },
    Add {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Begin(Vec<Expr>), // новый вариант
    Define {
        name: String,
        value: Box<Expr>,
    },
}

struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl Expr {
    fn var(&self) -> Option<&String> {
        match self {
            Expr::Var(name) => Some(name),
            _ => None,
        }
    }
}

impl<'a> Parser<'a> {
    fn new(input: &'a String) -> Self {
        Parser {
            input: input.as_bytes(),
            pos: 0,
        }
    }

    fn skip_ws_and_comments(&mut self) {
        loop {
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }
            if self.pos + 1 < self.input.len() && self.input[self.pos] == b';' {
                self.pos += 1;
                while self.pos < self.input.len() && self.input[self.pos] != b'\n' {
                    self.pos += 1;
                }
                if self.pos < self.input.len() && self.input[self.pos] == b'\n' {
                    self.pos += 1;
                }
            } else {
                break;
            }
        }
    }

    fn peek(&self) -> u8 {
        self.input.get(self.pos).copied().unwrap_or(b'\0')
    }

    fn consume(&mut self, expected: u8) -> Result<(), String> {
        self.skip_ws_and_comments();
        if self.peek() == expected {
            self.pos += 1;
            Ok(())
        } else {
            Err(format!(
                "Ожидалось '{}', пришло '{}'",
                expected as char,
                self.peek() as char
            ))
        }
    }

    fn parse(&mut self) -> Result<Expr, String> {
        self.skip_ws_and_comments();
        if self.peek() != b'(' {
            if self.peek().is_ascii_digit() || self.peek() == b'-' {
                return self.parse_num();
            }
            return self.parse_var();
        }

        self.pos += 1; // consume '('
        self.skip_ws_and_comments();

        let keyword = self.try_read_keyword();
        println!("Keyword: {:?}", keyword);
        let expr = match keyword.as_deref() {
            Some("λ") | Some("lambda") => self.parse_lam()?,
            Some("let") => self.parse_let()?,
            Some("if") => self.parse_if()?,
            Some("define") => {
                let name = self.parse_var()?.var().ok_or("имя для define")?.clone();
                let value = Box::new(self.parse_one_expr()?);
                self.consume(b')')?;
                Expr::Define { name, value }
            }
            Some("begin") => {
                let mut exprs = vec![];
                while self.peek() != b')' {
                    exprs.push(self.parse_one_expr()?);
                }
                self.consume(b')')?;
                Expr::Begin(exprs)
            }
            _ => {
                if self.peek() == b'+' {
                    self.pos += 1; // consume '+'
                    let first = self.parse_one_expr()?;
                    let second = self.parse_one_expr()?;
                    self.consume(b')')?;
                    Expr::Add {
                        left: Box::new(first),
                        right: Box::new(second),
                    }
                } else {
                    // Function application
                    println!("Parsing function application {:?}", self.peek() as char);
                    let first = if let Some(kw) = keyword {
                        // If we read a keyword, it is the first expression
                        match kw.as_str() {
                            _ => Expr::Var(kw),
                        }
                    } else {
                        self.parse_one_expr()?
                    };
                    // let first = self.parse_one_expr()?;

                    let mut args = vec![first];
                    while self.peek() != b')' {
                        args.push(self.parse_one_expr()?);
                    }
                    self.consume(b')')?;
                    let func = args.remove(0);
                    Expr::App {
                        func: Box::new(func),
                        args,
                    }
                }
            }
        };
        Ok(expr)
    }

    fn parse_one_expr(&mut self) -> Result<Expr, String> {
        self.skip_ws_and_comments();
        println!(
            "Parsing one expr at pos {}: '{}'",
            self.pos,
            self.peek() as char
        );
        if self.peek() == b'(' {
            self.parse()
        } else if self.peek().is_ascii_digit() || self.peek() == b'-' {
            self.parse_num()
        } else if self.peek().is_ascii_alphabetic() || self.peek() == b'_' {
            self.parse_var()
        } else {
            Err("Ожидалось атомарное выражение".into())
        }
    }

    fn parse_num(&mut self) -> Result<Expr, String> {
        self.skip_ws_and_comments();
        let start = self.pos;
        if self.peek() == b'-' {
            self.pos += 1;
        }
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        let s = std::str::from_utf8(&self.input[start..self.pos]).unwrap();
        s.parse::<i64>().map(Expr::Num).map_err(|e| e.to_string())
    }

    fn parse_var(&mut self) -> Result<Expr, String> {
        self.skip_ws_and_comments();
        let start = self.pos;
        while self.pos < self.input.len()
            && (self.input[self.pos].is_ascii_alphanumeric() || self.input[self.pos] == b'_')
        {
            self.pos += 1;
        }
        let name = std::str::from_utf8(&self.input[start..self.pos])
            .unwrap()
            .to_string();
        Ok(Expr::Var(name))
    }

    fn parse_lam(&mut self) -> Result<Expr, String> {
        self.skip_ws_and_comments();
        let param = self
            .parse_var()?
            .var()
            .ok_or("Ожидалось имя параметра".to_string())?
            .clone();
        self.skip_ws_and_comments();
        let body = Box::new(self.parse_one_expr()?);
        self.consume(b')')?;
        Ok(Expr::Lam { param, body: body })
    }

    fn parse_let(&mut self) -> Result<Expr, String> {
        self.skip_ws_and_comments();
        let name = self
            .parse_var()?
            .var()
            .ok_or("Ожидалось имя переменной".to_string())?
            .clone();
        println!("Let variable name: {}", name);
        self.skip_ws_and_comments();
        let value = self.parse_one_expr()?;
        println!("Let value parsed: {:?}", value);
        // let body = self.parse_one_expr()?;
        // body — всё остальное до )
        let mut body_exprs = vec![];
        while self.peek() != b')' {
            body_exprs.push(self.parse_one_expr()?);
        }
        self.consume(b')')?;

        let body = if body_exprs.len() == 1 {
            body_exprs.clone().into_iter().next().unwrap()
        } else {
            Expr::Begin(body_exprs.clone())
        };
        println!("Let body parsed: {:?} {:?}", body, body_exprs);
        // self.consume(b')')?;
        Ok(Expr::Let {
            name,
            value: Box::new(value),
            body: Box::new(body),
        })
    }

    fn parse_if(&mut self) -> Result<Expr, String> {
        self.skip_ws_and_comments();
        let cond = Box::new(self.parse_one_expr()?);
        self.skip_ws_and_comments();
        let then = Box::new(self.parse_one_expr()?);
        self.skip_ws_and_comments();
        let else_ = Box::new(self.parse_one_expr()?);
        self.consume(b')')?;
        Ok(Expr::If { cond, then, else_ })
    }

    fn try_read_keyword(&mut self) -> Option<String> {
        self.skip_ws_and_comments();
        let start = self.pos;
        while self.pos < self.input.len() && (self.input[self.pos].is_ascii_alphanumeric()) {
            self.pos += 1;
        }
        if start == self.pos {
            return None;
        }
        let keyword = std::str::from_utf8(&self.input[start..self.pos])
            .unwrap()
            .to_lowercase();
        Some(keyword)
    }
}

pub fn parse(input: &String) -> Result<Expr, String> {
    let mut parser = Parser::new(input);
    let e = parser.parse()?;
    parser.skip_ws_and_comments();
    if parser.pos < parser.input.len() {
        return Err("Неожиданные символы в конце ввода".into());
    }
    Ok(e)
}
