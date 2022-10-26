use std::{env, ffi::OsString};

pub struct Args;

impl Args {
    pub fn new() -> Vec<String> {
        let args = env::args_os()
            .map(OsString::into_string)
            .skip(1)
            .collect::<Result<Vec<String>, OsString>>();

        let mut args = match args {
            Ok(arg) => arg,
            Err(err) => panic!("{:?}", err),
        };

        if cfg!(target_os = "windows") {
            args.insert(0, "/C".to_string());
        } else {
            args.insert(0, "-c".to_string());
        };

        args
    }
}
