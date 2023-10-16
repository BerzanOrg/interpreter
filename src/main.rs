mod instruction;
mod value;
mod vm;

use instruction::Instruction;
use value::Value;

use crate::vm::VM;

fn main() {
    let mut vm = VM::new();

    let result = vm
        .run(vec![
            Instruction::DefineVariable(String::from("name"), Value::String(String::from("John"))),
            Instruction::DefineVariable(String::from("apples"), Value::Number(45)),
            Instruction::DefineVariable(String::from("bananas"), Value::Number(35)),
            Instruction::RunAndDefineVariable(
                String::from("total_fruits"),
                Box::new(Instruction::Add(vec![
                    String::from("bananas"),
                    String::from("apples"),
                ])),
            ),
            Instruction::DefineVariable(String::from("eaten_apples"), Value::Number(8)),
            Instruction::RunAndDefineVariable(
                String::from("total_fruits"),
                Box::new(Instruction::Subtract(vec![
                    String::from("total_fruits"),
                    String::from("eaten_apples"),
                ])),
            ),
            Instruction::DefineVariable(String::from("fruit_multiplier"), Value::Number(2)),
            Instruction::RunAndDefineVariable(
                String::from("total_fruits"),
                Box::new(Instruction::Multiply(vec![
                    String::from("total_fruits"),
                    String::from("fruit_multiplier"),
                ])),
            ),
            Instruction::DefineVariable(String::from("fruit_diversifier"), Value::Number(4)),
            Instruction::RunAndDefineVariable(
                String::from("total_fruits"),
                Box::new(Instruction::Diversify(vec![
                    String::from("total_fruits"),
                    String::from("fruit_diversifier"),
                ])),
            ),
        ])
        .unwrap();

    // result: 36
    println!("result: {}", result.to_string());
}
