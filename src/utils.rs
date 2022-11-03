use requestty::{ErrorKind, ListItem, Question};
use serde::Deserialize;

use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path, process};

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

pub fn read_json_file<P: AsRef<Path>>(path: P) -> Result<PackageJson, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `PackageJson`.
    let pkg_json: PackageJson = serde_json::from_reader(reader)?;

    // println!("{:?}", &pkg_json.scripts);
    // println!("{:?}", &pkg_json.packageManager);

    Ok(pkg_json)
}
