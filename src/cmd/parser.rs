use serde_json;

// command
// command subcommand
// command -options
// comamnd -k value
// command -k=value
// command --key=value
// command --key=element1,element2,element3
// command -k=e1 -k=e2
// command --key=e1 --key=e2
// command arg1 arg2 arg3
// command arg1 arg2 -k=value

pub fn parse(input: String) -> Result<(String, serde_json::Value), String> {
    let mut parts = input.split_whitespace();
    let mut command = String::new();
    let mut args = serde_json::Value::Object(serde_json::Map::new());

    loop {
        if let Some(part) = parts.next() {
            if command.is_empty() {
                command = part.to_string();
            } else {
                let arg_str = part.to_string();
                args = serde_json::Value::String(arg_str);
            }
        } else {
            break;
        }
    }

    Ok((command, args))
}
