use crate::codegen;
use crate::typechecker;
use std::collections::HashMap;
use std::fmt;

macro_rules! err {
    ($vm:expr, $msg:expr) => {{
        return Err(codegen::InterpreterError {
            err: $msg.to_string(),
            line: $vm.line,
            col: $vm.col,
        });
    }};
}

pub enum Opcode {
    Add,
    And,
    Arg(usize),
    Bconst(bool),
    Call,
    Div,
    Dup,
    Equal,
    Fconst(usize, HashMap<String, (usize, typechecker::Type)>),
    GetEnv(String),
    Greater,
    GreaterEqual,
    Iconst(i64),
    Jmp(i64),
    Jz(i64),
    Less,
    LessEqual,
    Mod,
    Mul,
    Not,
    NotEqual,
    Or,
    Pop,
    Recur(usize),
    Ret(usize),
    Rot,
    SetEnv(String),
    Srcpos(usize, usize),
    Sub,
    Uconst,
}

impl fmt::Display for Opcode {
    fn fmt<'a>(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::Add => write!(f, "add"),
            Opcode::And => write!(f, "and"),
            Opcode::Arg(n) => write!(f, "arg {}", n),
            Opcode::Bconst(b) => write!(f, "const {}", b),
            Opcode::Call => write!(f, "call"),
            Opcode::Div => write!(f, "div"),
            Opcode::Dup => write!(f, "dup"),
            Opcode::Equal => write!(f, "eq"),
            Opcode::Fconst(ip, _) => write!(f, "lambda @{}", ip),
            Opcode::GetEnv(id) => write!(f, "getenv {}", id),
            Opcode::Greater => write!(f, "gt"),
            Opcode::GreaterEqual => write!(f, "ge"),
            Opcode::Iconst(i) => write!(f, "const {}", i),
            Opcode::Jmp(ip) => write!(f, "jmp {}", ip),
            Opcode::Jz(ip) => write!(f, "jz {}", ip),
            Opcode::Less => write!(f, "lt"),
            Opcode::LessEqual => write!(f, "le"),
            Opcode::Mod => write!(f, "mod"),
            Opcode::Mul => write!(f, "mul"),
            Opcode::Not => write!(f, "not"),
            Opcode::NotEqual => write!(f, "neq"),
            Opcode::Or => write!(f, "or"),
            Opcode::Pop => write!(f, "pop"),
            Opcode::Recur(n) => write!(f, "recur {}", n),
            Opcode::Ret(n) => write!(f, "ret {}", n),
            Opcode::Rot => write!(f, "rot"),
            Opcode::SetEnv(id) => write!(f, "setenv {}", id),
            Opcode::Srcpos(line, col) => write!(f, "srcpos {} {}", line, col),
            Opcode::Sub => write!(f, "sub"),
            Opcode::Uconst => write!(f, "uconst"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    pub values: HashMap<String, Value>,
    pub types: HashMap<String, typechecker::Type>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            types: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Function(usize, Environment),
    Integer(i64),
    Tuple(Vec<Value>),
    Unit,
}

impl fmt::Display for Value {
    fn fmt<'a>(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Function(ip, _) => write!(f, "(lambda @{})", ip),
            Value::Integer(v) => write!(f, "{}", v),
            Value::Tuple(elements) => {
                write!(f, "(")?;
                for i in 0..elements.len() {
                    write!(f, "{}", elements[i])?;
                    if i + 1 != elements.len() {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Value::Unit => write!(f, "()"),
        }
    }
}

pub struct VirtualMachine {
    pub instructions: Vec<Opcode>,
    pub ip: usize,
    pub stack: Vec<Value>,
    pub callstack: Vec<(usize, Environment, usize, usize)>,

    pub env: Environment,

    pub line: usize,
    pub col: usize,
}

impl VirtualMachine {
    pub fn run(&mut self) -> Result<(), codegen::InterpreterError> {
        while self.ip < self.instructions.len() {
            match &self.instructions[self.ip] {
                Opcode::Add => match self.stack.pop() {
                    Some(Value::Integer(x)) => match self.stack.pop() {
                        Some(Value::Integer(y)) => {
                            self.stack.push(Value::Integer(x + y));
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                Opcode::And => match self.stack.pop() {
                    Some(Value::Boolean(x)) => match self.stack.pop() {
                        Some(Value::Boolean(y)) => {
                            self.stack.push(Value::Boolean(x && y));
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                Opcode::Arg(offset) => match self.callstack.last() {
                    Some((_, _, sp, _)) => {
                        self.stack.push(self.stack[*sp - offset].clone());
                    }
                    None => unreachable!(),
                },
                Opcode::Bconst(b) => {
                    self.stack.push(Value::Boolean(*b));
                }
                Opcode::Call => match self.stack.pop() {
                    Some(Value::Function(ip, env)) => {
                        let return_ip = self.ip;
                        self.ip = ip;
                        self.callstack
                            .push((ip, env, self.stack.len() - 1, return_ip));
                        continue;
                    }
                    _ => unreachable!(),
                },
                Opcode::Div => match self.stack.pop() {
                    Some(Value::Integer(x)) => match self.stack.pop() {
                        Some(Value::Integer(y)) => {
                            if y == 0 {
                                err!(self, "Division by zero.")
                            }
                            self.stack.push(Value::Integer(x / y));
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                Opcode::Dup => match self.stack.pop() {
                    Some(v) => {
                        self.stack.push(v.clone());
                        self.stack.push(v);
                    }
                    _ => unreachable!(),
                },
                Opcode::Equal => match self.stack.pop() {
                    Some(x) => match self.stack.pop() {
                        Some(y) => {
                            self.stack.push(Value::Boolean(x == y));
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                Opcode::Fconst(ip, upvalues) => {
                    let len = self.callstack.len();
                    let mut env;
                    if len > 0 {
                        env = self.callstack[len - 1].1.clone();
                    } else {
                        env = self.env.clone();
                    }
                    for upvalue in upvalues {
                        match self.callstack.last() {
                            Some((_, _, sp, _)) => {
                                let id = upvalue.0;
                                let offset = (upvalue.1).0;
                                let value = self.stack[*sp - offset].clone();
                                env.values.insert(id.to_string(), value);
                                env.types.insert(id.to_string(), (upvalue.1).1.clone());
                            }
                            None => {}
                        }
                    }
                    self.stack.push(Value::Function(*ip, env));
                }
                Opcode::GetEnv(id) => {
                    let len = self.callstack.len();
                    let values;
                    if len > 0 {
                        values = &self.callstack[len - 1].1.values;
                    } else {
                        values = &self.env.values;
                    }
                    match values.get(id) {
                        Some(x) => {
                            self.stack.push(x.clone());
                        }
                        None => unreachable!(),
                    }
                }
                Opcode::Greater => match self.stack.pop() {
                    Some(Value::Integer(x)) => match self.stack.pop() {
                        Some(Value::Integer(y)) => {
                            self.stack.push(Value::Boolean(x > y));
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                Opcode::GreaterEqual => match self.stack.pop() {
                    Some(Value::Integer(x)) => match self.stack.pop() {
                        Some(Value::Integer(y)) => {
                            self.stack.push(Value::Boolean(x >= y));
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                Opcode::Iconst(i) => {
                    self.stack.push(Value::Integer(*i));
                }
                Opcode::Jmp(offset) => {
                    self.ip = (self.ip as i64 + offset) as usize;
                    continue;
                }
                Opcode::Jz(offset) => match self.stack.pop() {
                    Some(Value::Boolean(v)) => {
                        if !v {
                            self.ip = (self.ip as i64 + offset) as usize;
                            continue;
                        }
                    }
                    _ => unreachable!(),
                },
                Opcode::Less => match self.stack.pop() {
                    Some(Value::Integer(x)) => match self.stack.pop() {
                        Some(Value::Integer(y)) => {
                            self.stack.push(Value::Boolean(x < y));
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                Opcode::LessEqual => match self.stack.pop() {
                    Some(Value::Integer(x)) => match self.stack.pop() {
                        Some(Value::Integer(y)) => {
                            self.stack.push(Value::Boolean(x <= y));
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                Opcode::Mod => match self.stack.pop() {
                    Some(Value::Integer(x)) => match self.stack.pop() {
                        Some(Value::Integer(y)) => {
                            if y == 0 {
                                err!(self, "Division by zero.")
                            }
                            self.stack.push(Value::Integer(x % y));
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                Opcode::Mul => match self.stack.pop() {
                    Some(Value::Integer(x)) => match self.stack.pop() {
                        Some(Value::Integer(y)) => {
                            self.stack.push(Value::Integer(x * y));
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                Opcode::NotEqual => match self.stack.pop() {
                    Some(x) => match self.stack.pop() {
                        Some(y) => {
                            self.stack.push(Value::Boolean(x != y));
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                Opcode::Not => match self.stack.pop() {
                    Some(Value::Boolean(x)) => {
                        self.stack.push(Value::Boolean(!x));
                    }
                    _ => unreachable!(),
                },
                Opcode::Or => match self.stack.pop() {
                    Some(Value::Boolean(x)) => match self.stack.pop() {
                        Some(Value::Boolean(y)) => {
                            self.stack.push(Value::Boolean(x || y));
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                Opcode::Pop => match self.stack.pop() {
                    Some(_) => {}
                    _ => unreachable!(),
                },
                Opcode::Recur(n) => match self.callstack.last() {
                    Some((ip, _, sp, _)) => {
                        for i in 0..*n {
                            match self.stack.pop() {
                                Some(v) => {
                                    self.stack[sp - (*n - i - 1)] = v;
                                }
                                _ => unreachable!(),
                            }
                        }
                        self.ip = *ip;
                    }
                    None => unreachable!(),
                },
                Opcode::Ret(n) => match self.callstack.pop() {
                    Some((_, _, sp, ip)) => {
                        self.stack.drain(sp..sp + n);
                        self.ip = ip;
                    }
                    None => unreachable!(),
                },
                Opcode::Rot => {
                    if self.stack.len() < 3 {
                        unreachable!();
                    }
                    if let Some(a) = self.stack.pop() {
                        self.stack.insert(self.stack.len() - 2, a);
                    }
                }
                Opcode::SetEnv(id) => match self.stack.pop() {
                    Some(x) => {
                        let len = self.callstack.len();
                        let values;
                        if len > 0 {
                            values = &mut self.callstack[len - 1].1.values;
                        } else {
                            values = &mut self.env.values;
                        }
                        values.insert(id.to_string(), x);
                    }
                    _ => unreachable!(),
                },
                Opcode::Srcpos(line, col) => {
                    self.line = *line;
                    self.col = *col;
                }
                Opcode::Sub => match self.stack.pop() {
                    Some(Value::Integer(x)) => match self.stack.pop() {
                        Some(Value::Integer(y)) => {
                            self.stack.push(Value::Integer(x - y));
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                Opcode::Uconst => {
                    self.stack.push(Value::Unit);
                }
            }
            self.ip += 1;
        }
        Ok(())
    }

    pub fn new() -> VirtualMachine {
        VirtualMachine {
            instructions: Vec::new(),
            ip: 0,
            stack: Vec::new(),
            callstack: Vec::new(),
            env: Environment::new(),
            line: usize::max_value(),
            col: usize::max_value(),
        }
    }
}
