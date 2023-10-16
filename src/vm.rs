use std::collections::HashMap;

use crate::{instruction::Instruction, value::Value};

/// Represents a virtual machine.
pub struct VM {
    variables: HashMap<String, Value>,
}

impl VM {
    /// Creates a new virtual machine.
    pub fn new() -> VM {
        VM {
            variables: HashMap::new(),
        }
    }

    /// Runs given instructions one by one and returns the result of the last instruction run.
    pub fn run(&mut self, instructions: Vec<Instruction>) -> Result<Value, ()> {
        let mut value = Value::Nothing;
        for instruction in instructions {
            value = self.run_instruction(instruction)?;
        }
        Ok(value)
    }

    /// Runs given instruction and returns its result.
    fn run_instruction(&mut self, instruction: Instruction) -> Result<Value, ()> {
        match instruction {
            Instruction::DefineVariable(name, value) => {
                self.variables.insert(name, value.clone());
                Ok(value)
            }
            Instruction::RunAndDefineVariable(name, instruction) => {
                let value = self.run_instruction(*instruction)?;
                self.variables.insert(name, value.clone());
                Ok(value)
            }
            Instruction::Add(names) => match names.first() {
                Some(name) => match self.variables.get(name) {
                    Some(value) => match value {
                        Value::Number(_) => {
                            let sum: i32 = names
                                .iter()
                                .map(|name| {
                                    self.variables.get(name).map(|value| match value {
                                        Value::Number(number) => Ok(*number),
                                        _ => Err(()),
                                    })
                                })
                                .flatten()
                                .flatten()
                                .reduce(|a, b| a + b)
                                .unwrap_or(0);

                            Ok(Value::Number(sum))
                        }
                        Value::String(_) => {
                            let sum: String = names
                                .iter()
                                .map(|name| {
                                    self.variables.get(name).map(|value| match value {
                                        Value::String(string) => Ok(string.clone()),
                                        _ => Err(()),
                                    })
                                })
                                .flatten()
                                .flatten()
                                .reduce(|a, b| a + &b)
                                .unwrap_or("".to_string());

                            Ok(Value::String(sum))
                        }
                        _ => Err(()),
                    },
                    None => Err(()),
                },
                None => Err(()),
            },
            Instruction::Subtract(names) => match names.first() {
                Some(name) => match self.variables.get(name) {
                    Some(value) => match value {
                        Value::Number(_) => {
                            let sub: i32 = names
                                .iter()
                                .map(|name| {
                                    self.variables.get(name).map(|value| match value {
                                        Value::Number(number) => Ok(*number),
                                        _ => Err(()),
                                    })
                                })
                                .flatten()
                                .flatten()
                                .reduce(|a, b| a - b)
                                .unwrap_or(0);

                            Ok(Value::Number(sub))
                        }
                        _ => Err(()),
                    },
                    None => Err(()),
                },
                None => Err(()),
            },
            Instruction::Multiply(names) => match names.first() {
                Some(name) => match self.variables.get(name) {
                    Some(value) => match value {
                        Value::Number(_) => {
                            let mul: i32 = names
                                .iter()
                                .map(|name| {
                                    self.variables.get(name).map(|value| match value {
                                        Value::Number(number) => Ok(*number),
                                        _ => Err(()),
                                    })
                                })
                                .flatten()
                                .flatten()
                                .reduce(|a, b| a * b)
                                .unwrap_or(0);

                            Ok(Value::Number(mul))
                        }
                        _ => Err(()),
                    },
                    None => Err(()),
                },
                None => Err(()),
            },
            Instruction::Diversify(names) => match names.first() {
                Some(name) => match self.variables.get(name) {
                    Some(value) => match value {
                        Value::Number(_) => {
                            let div: i32 = names
                                .iter()
                                .map(|name| {
                                    self.variables.get(name).map(|value| match value {
                                        Value::Number(number) => Ok(*number),
                                        _ => Err(()),
                                    })
                                })
                                .flatten()
                                .flatten()
                                .reduce(|a, b| a / b)
                                .unwrap_or(0);

                            Ok(Value::Number(div))
                        }
                        _ => Err(()),
                    },
                    None => Err(()),
                },
                None => Err(()),
            },
        }
    }
}
