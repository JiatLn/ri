use crate::error::CommonError;
use requestty::{ListItem, OnEsc, Question};
use serde::Deserialize;
use std::{collections::HashMap, fs, io::BufReader, path::Path, process};

pub fn exclude(args: Vec<String>, v: &str) -> Vec<String> {
    args.into_iter()
        .filter(|arg| arg != v)
        .collect::<Vec<String>>()
}

pub fn select_a_choice(
    vec_choices: &Vec<String>,
    name: &str,
    message: &str,
) -> Result<String, CommonError> {
    let select = Question::select(name)
        .message(message)
        .choices(vec_choices)
        .on_esc(OnEsc::Terminate)
        .transform(|choice, _previous_answers, backend| {
            write!(
                backend,
                "{}",
                choice.text.split(" - ").collect::<Vec<&str>>()[0]
            )
        })
        .build();

    let answer = requestty::prompt_one(select)?;

    match answer {
        requestty::Answer::ListItem(ListItem { text, .. }) => {
            let ans: Vec<&str> = text.split(" - ").collect();
            Ok(ans[0].to_string())
        }
        _ => process::exit(1),
    }
}

#[derive(Deserialize, Debug)]
pub struct PackageJson {
    pub scripts: Option<HashMap<String, String>>,
    // pub packageManager: Option<String>,
}

pub fn read_json_file<P: AsRef<Path>>(path: P) -> Result<PackageJson, CommonError> {
    let file = fs::File::open(path)?;

    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `PackageJson`.
    let pkg_json: PackageJson = serde_json::from_reader(reader)?;

    Ok(pkg_json)
}

pub fn remove_dir_all_file_with_path<P: AsRef<Path>>(path: P) -> Result<(), CommonError> {
    fs::remove_dir_all(path)?;
    Ok(())
}

pub fn ask_confirm_question(question_content: &str) -> Result<bool, CommonError> {
    let confirm = Question::confirm("q")
        .message(question_content)
        .default(false)
        .build();

    let confirm = requestty::prompt_one(confirm)?.as_bool();

    match confirm {
        Some(b) => Ok(b),
        None => Ok(false),
    }
}
