# interpreter
An simple interpreter.

## Example Usage
```rs
let mut vm = VM::new();

let result = vm
    .run(vec![
        Instruction::DefineVariable(
            String::from("apples"), 
            Value::Number(45)
        ),
        Instruction::DefineVariable(
            String::from("bananas"), 
            Value::Number(35)
        ),
        Instruction::RunAndDefineVariable(
            String::from("total_fruits"),
            Box::new(
                Instruction::Add(vec![
                    String::from("bananas"),
                    String::from("apples"),
                ])
            ),
        ),
    ])
    .unwrap();

    // result: 80
    println!("result: {}", result.to_string());
```
