use std::process;

use requestty::{ErrorKind, ListItem, Question};

pub fn exclude(args: Vec<String>, v: &str) -> Vec<String> {
    args.into_iter()
        .filter(|arg| arg != v)
        .collect::<Vec<String>>()
}

pub fn select_a_choice(
    vec_choices: &Vec<String>,
    name: &str,
    message: &str,
) -> Result<String, ErrorKind> {
    let select = Question::select(name)
        .message(message)
        .choices(vec_choices)
        .build();

    let answer = requestty::prompt_one(select)?;
    match answer {
        requestty::Answer::ListItem(ListItem { text, .. }) => Ok(text),
        _ => process::exit(1),
    }
}
