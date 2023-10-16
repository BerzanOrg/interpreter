use crate::value::Value;

/// Represents an instruction.
#[derive(Clone)]

pub enum Instruction {
    RunAndDefineVariable(String, Box<Instruction>),
    DefineVariable(String, Value),
    Add(Vec<String>),
    Subtract(Vec<String>),
    Multiply(Vec<String>),
    Diversify(Vec<String>),
}
