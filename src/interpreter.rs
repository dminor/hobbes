use crate::parser;
use crate::vm;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
enum Type {
    Boolean,
    Integer,
}

pub enum Value {
    Boolean(bool),
    Integer(i64),
}

impl fmt::Display for Value {
    fn fmt<'a>(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Integer(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug)]
pub struct InterpreterError {
    pub err: String,
    pub line: usize,
    pub col: usize,
}

impl fmt::Display for InterpreterError {
    fn fmt<'a>(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InterpreterError: {}", self.err)
    }
}

impl Error for InterpreterError {}

fn generate(ast: &parser::AST, vm: &mut vm::VirtualMachine, instr: &mut Vec<vm::Opcode>) {
    match ast {
        parser::AST::BinaryOp(op, lhs, rhs) => {
            generate(rhs, vm, instr);
            generate(lhs, vm, instr);
            match op {
                parser::Operator::And => {
                    instr.push(vm::Opcode::And);
                }
                parser::Operator::Divide => {
                    instr.push(vm::Opcode::Div);
                }
                parser::Operator::Equal => {
                    instr.push(vm::Opcode::Equal);
                }
                parser::Operator::Greater => {
                    instr.push(vm::Opcode::Greater);
                }
                parser::Operator::GreaterEqual => {
                    instr.push(vm::Opcode::GreaterEqual);
                }
                parser::Operator::Less => {
                    instr.push(vm::Opcode::Less);
                }
                parser::Operator::LessEqual => {
                    instr.push(vm::Opcode::LessEqual);
                }
                parser::Operator::Minus => {
                    instr.push(vm::Opcode::Sub);
                }
                parser::Operator::Mod => {
                    instr.push(vm::Opcode::Mod);
                }
                parser::Operator::Multiply => {
                    instr.push(vm::Opcode::Mul);
                }
                parser::Operator::Not => {
                    instr.push(vm::Opcode::Not);
                }
                parser::Operator::NotEqual => {
                    instr.push(vm::Opcode::NotEqual);
                }
                parser::Operator::Or => {
                    instr.push(vm::Opcode::Or);
                }
                parser::Operator::Plus => {
                    instr.push(vm::Opcode::Add);
                }
            }
        }
        parser::AST::Boolean(b) => {
            instr.push(vm::Opcode::Bconst(*b));
        }
        parser::AST::If(_, _) => {}
        parser::AST::Integer(i) => {
            instr.push(vm::Opcode::Iconst(*i));
        }
        parser::AST::UnaryOp(op, ast) => {
            generate(ast, vm, instr);
            match op {
                parser::Operator::Minus => {
                    instr.push(vm::Opcode::Iconst(0));
                    instr.push(vm::Opcode::Sub);
                }
                parser::Operator::Not => {
                    instr.push(vm::Opcode::Not);
                }
                _ => unreachable!(),
            }
        }
        parser::AST::None => {}
    }
}

fn typecheck(ast: &parser::AST) -> Result<Type, InterpreterError> {
    match ast {
        parser::AST::BinaryOp(op, lhs, rhs) => match typecheck(rhs) {
            Ok(rhs_type) => match typecheck(lhs) {
                Ok(lhs_type) => match op {
                    parser::Operator::Divide
                    | parser::Operator::Minus
                    | parser::Operator::Mod
                    | parser::Operator::Multiply
                    | parser::Operator::Plus => {
                        if rhs_type != Type::Integer || lhs_type != Type::Integer {
                            Err(InterpreterError {
                                err: "Type error: expected integer.".to_string(),
                                line: usize::max_value(),
                                col: usize::max_value(),
                            })
                        } else {
                            Ok(Type::Integer)
                        }
                    }
                    parser::Operator::Greater
                    | parser::Operator::GreaterEqual
                    | parser::Operator::Less
                    | parser::Operator::LessEqual => {
                        if rhs_type != Type::Integer || lhs_type != Type::Integer {
                            Err(InterpreterError {
                                err: "Type error: expected integer.".to_string(),
                                line: usize::max_value(),
                                col: usize::max_value(),
                            })
                        } else {
                            Ok(Type::Boolean)
                        }
                    }
                    parser::Operator::And | parser::Operator::Or => {
                        if rhs_type != Type::Boolean || lhs_type != Type::Boolean {
                            Err(InterpreterError {
                                err: "Type error: expected boolean.".to_string(),
                                line: usize::max_value(),
                                col: usize::max_value(),
                            })
                        } else {
                            Ok(Type::Boolean)
                        }
                    }
                    parser::Operator::Not => unreachable!(),
                    parser::Operator::Equal | parser::Operator::NotEqual => {
                        if rhs_type != lhs_type {
                            Err(InterpreterError {
                                err: "Type error: type mismatch.".to_string(),
                                line: usize::max_value(),
                                col: usize::max_value(),
                            })
                        } else {
                            Ok(Type::Boolean)
                        }
                    }
                },
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        },
        parser::AST::Boolean(_) => Ok(Type::Boolean),
        parser::AST::If(_, _) => Ok(Type::Integer),
        parser::AST::Integer(_) => Ok(Type::Integer),
        parser::AST::UnaryOp(op, ast) => match typecheck(ast) {
            Ok(ast_type) => match op {
                parser::Operator::Minus => {
                    if ast_type == Type::Integer {
                        Ok(Type::Integer)
                    } else {
                        Err(InterpreterError {
                            err: "Type error: expected integer.".to_string(),
                            line: usize::max_value(),
                            col: usize::max_value(),
                        })
                    }
                }
                parser::Operator::Not => {
                    if ast_type == Type::Boolean {
                        Ok(Type::Boolean)
                    } else {
                        Err(InterpreterError {
                            err: "Type error: expected boolean.".to_string(),
                            line: usize::max_value(),
                            col: usize::max_value(),
                        })
                    }
                }
                _ => Err(InterpreterError {
                    err: "Invalid unary operator.".to_string(),
                    line: usize::max_value(),
                    col: usize::max_value(),
                }),
            },
            Err(err) => Err(err),
        },
        parser::AST::None => Err(InterpreterError {
            err: "None has no type.".to_string(),
            line: usize::max_value(),
            col: usize::max_value(),
        }),
    }
}

pub fn eval(vm: &mut vm::VirtualMachine, ast: &parser::AST) -> Result<Value, InterpreterError> {
    match typecheck(ast) {
        Ok(typ) => {
            let mut instr = Vec::new();
            generate(ast, vm, &mut instr);
            vm.ip = vm.instructions.len();
            vm.instructions.extend(instr);
            match vm.run() {
                Ok(()) => match vm.stack.pop() {
                    Some(v) => match typ {
                        Type::Boolean => Ok(Value::Boolean(v != 0)),
                        Type::Integer => Ok(Value::Integer(v)),
                    },
                    None => Err(InterpreterError {
                        err: "Stack underflow.".to_string(),
                        line: usize::max_value(),
                        col: usize::max_value(),
                    }),
                },
                Err(e) => Err(e),
            }
        }
        Err(err) => {
            return Err(err);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter;
    use crate::parser;
    use crate::vm;

    macro_rules! eval {
        ($input:expr, $type:tt, $value:expr) => {{
            let mut vm = vm::VirtualMachine::new();
            match parser::parse($input) {
                parser::ParseResult::Matched(ast, _) => match interpreter::eval(&mut vm, &ast) {
                    Ok(v) => match v {
                        interpreter::Value::$type(t) => {
                            assert_eq!(t, $value);
                        }
                        _ => {
                            assert!(false);
                        }
                    },
                    Err(_) => {
                        assert!(false);
                    }
                },
                parser::ParseResult::NotMatched(_) => {
                    assert!(false);
                }
                parser::ParseResult::Error(_, _, _) => {
                    assert!(false);
                }
            }
        }};
    }

    macro_rules! evalfails {
        ($input:expr, $err:expr) => {{
            let mut vm = vm::VirtualMachine::new();
            match parser::parse($input) {
                parser::ParseResult::Matched(ast, _) => match interpreter::eval(&mut vm, &ast) {
                    Ok(_) => {
                        assert!(false);
                    }
                    Err(err) => {
                        assert_eq!(err.err, $err);
                    }
                },
                parser::ParseResult::NotMatched(_) => {
                    assert!(false);
                }
                parser::ParseResult::Error(_, _, _) => {
                    assert!(false);
                }
            }
        }};
    }

    macro_rules! typecheck {
        ($input:expr, $value:expr) => {{
            match parser::parse($input) {
                parser::ParseResult::Matched(ast, _) => match interpreter::typecheck(&ast) {
                    Ok(typ) => {
                        assert_eq!(typ, $value);
                    }
                    Err(_) => {
                        assert!(false);
                    }
                },
                parser::ParseResult::NotMatched(_) => {
                    assert!(false);
                }
                parser::ParseResult::Error(_, _, _) => {
                    assert!(false);
                }
            }
        }};
    }

    #[test]
    fn evals() {
        eval!("1 + 2", Integer, 3);
        eval!("1 - 2", Integer, -1);
        eval!("1 * 2", Integer, 2);
        eval!("4 / 2", Integer, 2);
        eval!("true && false", Boolean, false);
        eval!("true || false", Boolean, true);
        eval!("21 % 6", Integer, 3);
        eval!("!true", Boolean, false);
        eval!("-42", Integer, -42);
        eval!("1 < 2", Boolean, true);
        eval!("2 <= 2", Boolean, true);
        eval!("2 == 2", Boolean, true);
        eval!("2 != 2", Boolean, false);
        eval!("1 > 2", Boolean, false);
        eval!("2 >= 2", Boolean, true);
        eval!("5 * 4 * 3 * 2 * 1", Integer, 120);
        typecheck!("5", interpreter::Type::Integer);
        typecheck!("true", interpreter::Type::Boolean);
        typecheck!("2 + 5 + 3", interpreter::Type::Integer);
        typecheck!("true && false", interpreter::Type::Boolean);
        typecheck!("!false", interpreter::Type::Boolean);
        typecheck!("-1", interpreter::Type::Integer);
        evalfails!("1 + true", "Type error: expected integer.");
        evalfails!("1 && true", "Type error: expected boolean.");
        evalfails!("!1", "Type error: expected boolean.");
        evalfails!("-false", "Type error: expected integer.");
        evalfails!("1 == true", "Type error: type mismatch.");
        evalfails!("1 != false", "Type error: type mismatch.");
        evalfails!("0 <= false", "Type error: expected integer.");
        eval!("(1 + 2) * 5", Integer, 15);
        eval!("1 + 2 * 5", Integer, 11);
        evalfails!("1 / 0", "Division by zero.");
        evalfails!("1 % 0", "Division by zero.");
    }
}
