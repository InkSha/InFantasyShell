pub fn parse(input: &str) -> Result<(String, Vec<String>), String> {
    let mut parts = input.split_whitespace();
    let command = parts
        .next()
        .ok_or_else(|| "empty command".to_string())?
        .to_string();
    let args = parts.map(ToString::to_string).collect::<Vec<_>>();

    Ok((command, args))
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn splits_command_and_arguments() {
        let (command, args) = parse("echo hello world > notes.txt").expect("parse should succeed");

        assert_eq!(command, "echo");
        assert_eq!(args, vec!["hello", "world", ">", "notes.txt"]);
    }
}
