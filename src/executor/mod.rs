pub mod ast;
mod datatype;

use ast::*;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

use crate::lexer::tokens::TokenType;
use crate::executor::datatype::{DataTypes,to_bool};

pub struct Executor {
    symbol_table: HashMap<String, DataTypes>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            symbol_table: HashMap::new(),
        }
    }

    pub fn execute(&mut self, unit: &SourceUnit) {
        for x in &unit.0 {
            let SourceUnitPart::Statement(stmt) = x;
            match stmt {
                Statement::Declaration(symbol) => {
                    if let Expression::Symbol(name) = symbol {
                        if !self
                            .symbol_table
                            .insert(name.to_string(), DataTypes::Unknown)
                            .is_none()
                        {
                            panic!("Symbol {} already defiined", name)
                        }
                    } else {
                        panic!("Invalid declaration statement!")
                    }
                }
                Statement::Conditional(expr, truebody, falsebody) => {
                    let truth = to_bool(self.eval_arithmetic_logic_expression(expr).unwrap());
                    if truth {
                        self.execute(&truebody);
                    } else {
                        if let Some(body) = falsebody {
                            self.execute(&body);
                        }
                    }
                }
                Statement::Loop(expr, body) => {
                    let mut truth = to_bool(self.eval_arithmetic_logic_expression(expr).unwrap());
                    while truth {
                        self.execute(&body);
                        truth = to_bool(self.eval_arithmetic_logic_expression(expr).unwrap());
                    }
                }
                Statement::Write(expr) => {
                    print!("{}", self.eval_arithmetic_logic_expression(expr).unwrap());
                    let _ = stdout().flush();
                }
                Statement::Assignment(l, r) => {
                    if let Expression::Symbol(identifier) = l {
                        if !self.symbol_table.contains_key(&identifier.to_string()) {
                            panic!("Undefined symbol {}", identifier);
                        }
                        self.symbol_table.insert(
                            identifier.to_string(),
                            self.eval_arithmetic_logic_expression(&*r).unwrap(),
                        );
                    } else {
                        panic!("Invalid left assignment operator expected a symbol")
                    }
                }
            }
        }
    }

    fn eval_arithmetic_logic_expression(
        &self,
        expr: &Expression,
    ) -> Result<DataTypes, &'static str> {
        match expr {
            Expression::Add(l, r) => Ok(self.eval_arithmetic_logic_expression(&**l).unwrap()
                + self.eval_arithmetic_logic_expression(&**r).unwrap()),
            Expression::Multiply(l, r) => Ok(self.eval_arithmetic_logic_expression(&**l).unwrap()
                * self.eval_arithmetic_logic_expression(&**r).unwrap()),
            Expression::Subtract(l, r) => Ok(self.eval_arithmetic_logic_expression(&**l).unwrap()
                - self.eval_arithmetic_logic_expression(&**r).unwrap()),
            Expression::Divide(l, r) => Ok(self.eval_arithmetic_logic_expression(&**l).unwrap()
                / self.eval_arithmetic_logic_expression(&**r).unwrap()),
            Expression::UnaryMinus(r) => {
                Ok(DataTypes::Integer(-1) * self.eval_arithmetic_logic_expression(&**r).unwrap())
            }
            Expression::Equals(l, r) => Ok(DataTypes::Bool(
                self.eval_arithmetic_logic_expression(&**l).unwrap()
                    == self.eval_arithmetic_logic_expression(&**r).unwrap(),
            )),
            Expression::NotEquals(l, r) => Ok(DataTypes::Bool(
                self.eval_arithmetic_logic_expression(&**l).unwrap()
                    != self.eval_arithmetic_logic_expression(&**r).unwrap(),
            )),

            Expression::GreaterThan(l, r) => Ok(DataTypes::Bool(
                self.eval_arithmetic_logic_expression(&**l).unwrap()
                    > self.eval_arithmetic_logic_expression(&**r).unwrap(),
            )),

            Expression::LessThan(l, r) => Ok(DataTypes::Bool(
                self.eval_arithmetic_logic_expression(&**l).unwrap()
                    < self.eval_arithmetic_logic_expression(&**r).unwrap(),
            )),
            Expression::Integer(l) => match l {
                TokenType::Number(number) => Ok(DataTypes::Integer(*number)),
                _ => Err("Invalid constant"),
            },
            Expression::Symbol(identifier) => {
                if !self.symbol_table.contains_key(&identifier.to_string()) {
                    panic!("Undefined symbol {}", identifier);
                }
                match self.symbol_table.get(&identifier.to_string()).unwrap() {
                    DataTypes::Integer(number) => Ok(DataTypes::Integer(*number)),
                    DataTypes::String(data) => Ok(DataTypes::String(data.to_string())),
                    _ => Err("Invalid constant in expression"),
                }
                
            }
            Expression::StringLiteral(value) => {
                Ok(DataTypes::String(value.to_string()))
            }
            Expression::InputNumber => {
                let mut input = String::new();
                stdin().read_line(&mut input).expect("Unable to read input");
                Ok(DataTypes::Integer(input.trim().parse().expect("Invalid integer input")))
            }
            Expression::InputString => {
                let mut input = String::new();
                stdin().read_line(&mut input).expect("Unable to read input");
                Ok(DataTypes::String(input))
            }
        }
    }

    
}
