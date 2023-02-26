pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];

    for ch in string.chars() {
        if is_opening_bracket(ch) {
            stack.push(ch);
        } else {
            if let Some(b) = closing_to_opening(ch) {
                if stack.is_empty() || stack.last().copied().unwrap() != b {
                    return false;
                }
                stack.pop();
            }
        }
    }

    stack.is_empty()
}

fn closing_to_opening(closing_bracket: char) -> Option<char> {
    match closing_bracket {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        _ => None,
    }
}

fn is_opening_bracket(input: char) -> bool {
    match input {
        '(' | '[' | '{' => true,
        _ => false,
    }
}
