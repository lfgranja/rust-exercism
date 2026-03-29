pub fn reply(message: &str) -> &str {
    match message.trim() {
        "" => "Fine. Be that way!",
        msg => match (
            // is a question -> ends with a question mark
            msg.ends_with('?'),
            // is yelling -> all UPPERCASE and contains at least one letter
            !msg.contains(char::is_lowercase) && msg.contains(char::is_alphabetic),
        ) {
            (true, true) => "Calm down, I know what I'm doing!",
            (false, true) => "Whoa, chill out!",
            (true, false) => "Sure.",
            (false, false) => "Whatever.",
        },
    }
}
