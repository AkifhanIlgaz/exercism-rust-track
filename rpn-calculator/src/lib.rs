#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];

    for input in inputs {
        match input {
            CalculatorInput::Value(val) => stack.push(*val),
            _ => {
                let second_operand = stack.pop();
                let first_operand = stack.pop();

                match (first_operand, second_operand) {
                    (Some(first_op), Some(second_op)) => match input {
                        CalculatorInput::Add => stack.push(first_op + second_op),
                        CalculatorInput::Subtract => stack.push(first_op - second_op),
                        CalculatorInput::Multiply => stack.push(first_op * second_op),
                        CalculatorInput::Divide => stack.push(first_op / second_op),
                        _ => return None,
                    },
                    _ => return None,
                }
            }
        }
    }

    match stack.len() {
        1 => stack.pop(),
        _ => None,
    }
}
