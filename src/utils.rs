pub fn exclude(args: Vec<String>, v: &str) -> Vec<String> {
    args.into_iter()
        .filter(|arg| arg != v)
        .collect::<Vec<String>>()
}
