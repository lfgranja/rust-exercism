//! rust/bob/src/lib.rs

/*
Introduction
Bob is a lackadaisical teenager. He likes to think that he's very cool. And he definitely doesn't get excited about things. That wouldn't be cool.

When people talk to him, his responses are pretty limited.

Instructions
Your task is to determine what Bob will reply to someone when they say something to him or ask him a question.

Bob only ever answers one of five things:

"Sure." This is his response if you ask him a question, such as "How are you?" The convention used for questions is that it ends with a question mark.
"Whoa, chill out!" This is his answer if you YELL AT HIM. The convention used for yelling is ALL CAPITAL LETTERS.
"Calm down, I know what I'm doing!" This is what he says if you yell a question at him.
"Fine. Be that way!" This is how he responds to silence. The convention used for silence is nothing, or various combinations of whitespace characters.
"Whatever." This is what he answers to anything else.
Source
Inspired by the 'Deaf Grandma' exercise in Chris Pine's Learn to Program tutorial.The link opens in a new window or tab
*/

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
