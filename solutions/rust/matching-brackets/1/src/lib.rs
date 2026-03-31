pub fn brackets_are_balanced(string: &str) -> bool {
    string
        .chars()
        .try_fold(Vec::new(), |mut stack, char| {
            match char {
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                ']' | '}' | ')' => {
                    // If what is popped from the stack doesn't match the current closing bracket, force return Error
                    if stack.pop() != Some(char) {
                        return Err(());
                    }
                }
                _ => (),
            }
            Ok(stack)
        })
        .is_ok_and(|stack| stack.is_empty())
}
